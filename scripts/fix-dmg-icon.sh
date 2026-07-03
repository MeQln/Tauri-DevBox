#!/usr/bin/env bash
# scripts/fix-dmg-icon.sh
#
# 把 tauri build 产出的 .dmg 文件的 Finder 图标修正为与 .app 一致。
#
# 根因：Tauri 2 的 create-dmg 流程用 `SetFile -c icnC .VolumeIcon.icns` 设置的是
# 卷内文件的 custom icon 属性，这只影响挂载后 volume 的 Finder 图标；.dmg 文件
# 本身在 Finder 显示的图标由 com.apple.ResourceFork + com.apple.FinderInfo xattr
# 决定，create-dmg 不会写入这俩 xattr，所以 .dmg 在 Finder 显示系统默认磁盘图。
#
# 修复：sips -i 能把 .icns 按正确的 resource fork 格式写入 com.apple.ResourceFork，
# 但 sips 会覆盖目标文件。所以用临时文件做中转：先让 sips 把 icon 写入临时 .dmg
# （内容被覆盖无所谓），再把临时文件的两条 xattr 拷回原始 .dmg。
#
# 仅在 macOS 上执行；非 macOS / 找不到产物时静默跳过；任何错误也不影响 build 退出码。

set -uo pipefail

main() {
  if [[ "$(uname)" != "Darwin" ]]; then
    return 0
  fi

  local script_dir project_root dmg_dir icon
  script_dir="$(cd "$(dirname "$0")" && pwd)"
  project_root="$(cd "$script_dir/.." && pwd)"
  dmg_dir="$project_root/src-tauri/target/release/bundle/dmg"
  icon="$project_root/src-tauri/icons/icon.icns"

  [[ -d "$dmg_dir" ]] || return 0
  [[ -f "$icon" ]] || return 0

  shopt -s nullglob
  local dmgs=("$dmg_dir"/*.dmg)
  shopt -u nullglob
  [[ ${#dmgs[@]} -gt 0 ]] || return 0

  local tmp_dmg tmp_rf tmp_fi
  tmp_dmg="$(mktemp -t fix-dmg-icon).dmg"
  tmp_rf="$(mktemp -t fix-dmg-icon-rf)"
  tmp_fi="$(mktemp -t fix-dmg-icon-fi)"

  # sips -i 把 .icns 写入新文件的同时，按 macOS resource fork 格式写入 com.apple.ResourceFork xattr
  sips -i "$icon" --out "$tmp_dmg" >/dev/null 2>&1

  xattr -px com.apple.ResourceFork "$tmp_dmg" > "$tmp_rf"
  xattr -px com.apple.FinderInfo "$tmp_dmg" > "$tmp_fi" 2>/dev/null || printf '' > "$tmp_fi"

  for dmg in "${dmgs[@]}"; do
    xattr -wx com.apple.ResourceFork "$(cat "$tmp_rf")" "$dmg"
    xattr -wx com.apple.FinderInfo "$(cat "$tmp_fi")" "$dmg"
    echo "[fix-dmg-icon] $dmg"
  done

  rm -f "$tmp_dmg" "$tmp_rf" "$tmp_fi"
}

if ! main; then
  echo "[fix-dmg-icon] 修复失败（不影响 build）" >&2
fi
exit 0
