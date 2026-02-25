import { defineConfig } from 'vitepress'
import { resolveDocsBase } from "../../../docs-hub/.vitepress/base.config"

const docsBase = resolveDocsBase()

export default defineConfig({
  title: 'TokenLedger',
  description: 'Token and cost tracking for LLM operations',
  base: docsBase,
  ignoreDeadLinks: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Specs', link: '/SPEC' },
      { text: 'PRD', link: '/PRD' },
    ],
    sidebar: [
      {
        text: 'Documentation',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Specs', link: '/SPEC' },
          { text: 'PRD', link: '/PRD' },
          { text: 'Changelog', link: '/CHANGELOG' },
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/kooshapari/tokenledger' }
    ]
  }
})
