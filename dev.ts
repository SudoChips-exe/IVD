// dev.ts — Orchestrate backend (Rust) and frontend (Bun) concurrently
// Run with: `bun run dev` from the project root

function prefixLog(prefix: string, text: string) {
  for (const line of text.split(/\r?\n/)) {
    if (line.length > 0) console.log(`${prefix} ${line}`)
  }
}

async function streamReader(name: string, stream: ReadableStream<Uint8Array> | null) {
  if (!stream) return
  const reader = stream.getReader()
  const decoder = new TextDecoder()
  let buf = ''
  while (true) {
    const { value, done } = await reader.read()
    if (done) break
    buf += decoder.decode(value, { stream: true })
    const lines = buf.split(/\r?\n/)
    buf = lines.pop() || ''
    for (const line of lines) prefixLog(`[${name}]`, line)
  }
  if (buf) prefixLog(`[${name}]`, buf)
}

function killChild(child: any) {
  try {
    child.kill('SIGTERM')
  } catch (e) {
    // ignore
  }
}

async function main() {
  console.log('Starting both backend and frontend...')

  // Check if cargo is available. If not, run only the frontend and print install steps.
  let cargoAvailable = true
  try {
    const check = Bun.spawnSync({ cmd: ['cargo', '--version'] })
    if (check.exitCode !== 0) cargoAvailable = false
  } catch (e) {
    cargoAvailable = false
  }

  if (!cargoAvailable) {
    console.warn('[orchestrator] `cargo` not found in PATH — starting frontend only.')
    console.warn('[orchestrator] To enable backend orchestration, install Rust (https://www.rust-lang.org/tools/install)')

    // Ensure frontend dependencies are installed before running dev
    try {
      console.log('[orchestrator] Running `bun install` in frontend (if needed)')
      const install = Bun.spawnSync({ cmd: ['bun', 'install'], cwd: 'frontend' })
      if (install.exitCode !== 0) {
        console.error('[orchestrator] `bun install` failed:')
        if (install.stdout) prefixLog('[bun install]', new TextDecoder().decode(install.stdout))
        if (install.stderr) prefixLog('[bun install-err]', new TextDecoder().decode(install.stderr))
        process.exit(install.exitCode || 1)
      }
    } catch (e) {
      console.error('[orchestrator] Failed to run `bun install` in frontend:', e)
      process.exit(1)
    }

    const frontendOnly = Bun.spawn({
      cmd: ['bun', 'run', 'dev'],
      cwd: 'frontend',
      stdout: 'pipe',
      stderr: 'pipe',
    })

    streamReader('frontend', frontendOnly.stdout)
    streamReader('frontend-err', frontendOnly.stderr)

    const f = await frontendOnly.exited
    if (f !== 0) {
      console.error('[orchestrator] Frontend exited with code', f)
      process.exit(f || 1)
    }

    console.log('Frontend exited normally')
    return
  }

  const backend = Bun.spawn({
    cmd: ['cargo', 'run'],
    cwd: 'backend',
    stdout: 'pipe',
    stderr: 'pipe',
  })

  // Ensure frontend dependencies are installed before running dev
  try {
    console.log('[orchestrator] Running `bun install` in frontend (if needed)')
    const install = Bun.spawnSync({ cmd: ['bun', 'install'], cwd: 'frontend' })
    if (install.exitCode !== 0) {
      console.error('[orchestrator] `bun install` failed:')
      if (install.stdout) prefixLog('[bun install]', new TextDecoder().decode(install.stdout))
      if (install.stderr) prefixLog('[bun install-err]', new TextDecoder().decode(install.stderr))
      killChild(backend)
      process.exit(install.exitCode || 1)
    }
  } catch (e) {
    console.error('[orchestrator] Failed to run `bun install` in frontend:', e)
    killChild(backend)
    process.exit(1)
  }

  const frontend = Bun.spawn({
    cmd: ['bun', 'run', 'dev'],
    cwd: 'frontend',
    stdout: 'pipe',
    stderr: 'pipe',
  })

  // Pipe logs
  streamReader('backend', backend.stdout)
  streamReader('backend-err', backend.stderr)
  streamReader('frontend', frontend.stdout)
  streamReader('frontend-err', frontend.stderr)

  // Handle signals to shut down both children
  for (const sig of ['SIGINT', 'SIGTERM', 'SIGHUP']) {
    try {
      process.on(sig as any, () => {
        console.log(`Received ${sig}, shutting down children...`)
        killChild(backend)
        killChild(frontend)
        process.exit(0)
      })
    } catch (e) {
      // Some platforms may not support all signals
    }
  }

  const [b, f] = await Promise.all([backend.exited, frontend.exited])

  if (b !== 0) {
    console.error('[orchestrator] Backend exited with code', b)
    killChild(frontend)
    process.exit(b || 1)
  }

  if (f !== 0) {
    console.error('[orchestrator] Frontend exited with code', f)
    killChild(backend)
    process.exit(f || 1)
  }

  console.log('Both processes exited normally')
}

main().catch((err) => {
  console.error('Orchestrator error:', err)
  process.exit(1)
})
