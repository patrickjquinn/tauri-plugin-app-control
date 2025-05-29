import { readFileSync } from 'fs'
import { join, resolve } from 'path'
import { cwd } from 'process'
import typescript from '@rollup/plugin-typescript'

const pkg = JSON.parse(readFileSync(join(cwd(), 'package.json'), 'utf8'))

export default {
  input: 'guest-js/index.ts',
  output: [
    {
      file: pkg.exports['.'].import,
      format: 'esm'
    },
    {
      file: pkg.exports['.'].require,
      format: 'cjs'
    }
  ],
  plugins: [
    typescript({
      tsconfig: resolve(cwd(), 'guest-js/tsconfig.json'),
      rootDir: resolve(cwd(), 'guest-js'),
      include: ['index.ts'],
      exclude: [
        resolve(cwd(), 'node_modules/**'),
        resolve(cwd(), 'dist/**'),
        resolve(cwd(), 'webview-src/**')
      ],
      declaration: true
    })
  ],
  external: [
    /^@tauri-apps\/api/,
    ...Object.keys(pkg.dependencies || {}),
    ...Object.keys(pkg.peerDependencies || {})
  ]
}
