import type { Config } from 'tailwindcss'

export default {
  content: ['./index.html', './src/**/*.{vue,ts}'],
  theme: {
    extend: {
      colors: {
        bg:        'var(--bg)',
        surface:   'var(--surface)',
        aside:     'var(--aside)',
        'aside-2': 'var(--aside-2)',
        card:      'var(--card)',
        rule:      'var(--rule)',
        ink:       'var(--ink)',
        'ink-2':   'var(--ink-2)',
        'ink-3':   'var(--ink-3)',
        amber:     'var(--amber)',
        link:      'var(--link)',
      },
      borderRadius: {
        sm: 'var(--r-sm)',
        md: 'var(--r-md)',
        lg: 'var(--r-lg)',
      },
      fontFamily: {
        sans: ['var(--sans)'],
        mono: ['var(--mono)'],
      },
    },
  },
  plugins: [],
} satisfies Config
