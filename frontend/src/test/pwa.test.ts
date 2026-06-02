// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync, existsSync, readdirSync } from 'node:fs'
import { resolve, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const distDir = resolve(__dirname, '../../dist')
const publicDir = resolve(__dirname, '../../public')

describe('PWA manifest', () => {
  const manifestPath = resolve(distDir, 'manifest.webmanifest')

  it('manifest.webmanifest exists in dist', () => {
    expect(existsSync(manifestPath)).toBe(true)
  })

  it('manifest has required fields', () => {
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf-8'))
    expect(manifest.name).toBeTruthy()
    expect(manifest.short_name).toBeTruthy()
    expect(manifest.start_url).toBe('/')
    expect(manifest.display).toBe('standalone')
    expect(manifest.theme_color).toBeTruthy()
    expect(manifest.background_color).toBeTruthy()
  })

  it('manifest has at least two icons', () => {
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf-8'))
    expect(manifest.icons.length).toBeGreaterThanOrEqual(2)
  })

  it('manifest icons have required sizes (192 and 512)', () => {
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf-8'))
    const sizes = manifest.icons.map((i: { sizes: string }) => i.sizes)
    expect(sizes).toContain('192x192')
    expect(sizes).toContain('512x512')
  })

  it('manifest has maskable icon', () => {
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf-8'))
    const hasMaskable = manifest.icons.some((i: { purpose?: string }) =>
      i.purpose?.includes('maskable')
    )
    expect(hasMaskable).toBe(true)
  })

  it('short_name is VIDCLAW', () => {
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf-8'))
    expect(manifest.short_name).toBe('VIDCLAW')
  })
})

describe('PWA service worker', () => {
  it('sw.js exists in dist', () => {
    expect(existsSync(resolve(distDir, 'sw.js'))).toBe(true)
  })

  it('workbox runtime file exists in dist', () => {
    const files = readdirSync(distDir) as string[]
    const hasWorkbox = files.some((f: string) => f.startsWith('workbox-') && f.endsWith('.js'))
    expect(hasWorkbox).toBe(true)
  })
})

describe('PWA icons', () => {
  it('icon-192.png exists in public', () => {
    expect(existsSync(resolve(publicDir, 'icon-192.png'))).toBe(true)
  })

  it('icon-512.png exists in public', () => {
    expect(existsSync(resolve(publicDir, 'icon-512.png'))).toBe(true)
  })

  it('icon-192.png is valid PNG (correct header bytes)', () => {
    const buf = readFileSync(resolve(publicDir, 'icon-192.png'))
    // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
    expect(buf[0]).toBe(0x89)
    expect(buf[1]).toBe(0x50) // P
    expect(buf[2]).toBe(0x4e) // N
    expect(buf[3]).toBe(0x47) // G
  })

  it('icon-512.png is non-empty', () => {
    const buf = readFileSync(resolve(publicDir, 'icon-512.png'))
    expect(buf.length).toBeGreaterThan(100)
  })
})
