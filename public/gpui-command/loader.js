const status = document.querySelector('#status');
const params = new URLSearchParams(location.search);
const demo = params.get('demo') || 'button';
const dark = params.get('theme') === 'dark';
document.title = `${demo[0].toUpperCase() + demo.slice(1)} · GPUI`;
// Embedded examples should receive keyboard focus only after visitor interaction.
const observer = new MutationObserver((records) => {
  for (const record of records) for (const node of record.addedNodes) {
    if ((node instanceof HTMLInputElement || node instanceof HTMLTextAreaElement) && window.parent !== window && document.activeElement === node) node.blur();
  }
});
if (document.body) observer.observe(document.body, { childList: true });

// The WASM entry point returns before asynchronous GPU initialization finishes.
// Keep one loader throughout startup, until GPUI has actually drawn a frame.
function startPreview(run) {
  return new Promise((resolve, reject) => {
    function cleanup() {
      clearTimeout(timeout);
      graphicsErrors.disconnect();
      window.removeEventListener('gpui-preview-ready', ready);
      window.removeEventListener('error', failed);
      window.removeEventListener('unhandledrejection', failed);
    }
    function ready() {
      cleanup();
      resolve();
    }
    function failed(event) {
      cleanup();
      reject(event.error || event.reason || new Error(event.message || 'The preview could not initialize.'));
    }
    const timeout = setTimeout(() => {
      cleanup();
      reject(new Error('The preview took too long to start. Please try again.'));
    }, 60000);
    // GPUI reports adapter/device startup failures in the document rather than
    // throwing them out of its asynchronous platform initialization.
    const graphicsErrors = new MutationObserver((records) => {
      for (const record of records) for (const node of record.addedNodes) {
        if (node instanceof HTMLParagraphElement && node.textContent.startsWith('Failed to initialize browser graphics:')) {
          const message = node.textContent;
          node.remove();
          failed({ message });
          return;
        }
      }
    });
    graphicsErrors.observe(document.body, { childList: true });
    window.addEventListener('gpui-preview-ready', ready);
    window.addEventListener('error', failed);
    window.addEventListener('unhandledrejection', failed);
    try {
      run();
    } catch (error) {
      cleanup();
      reject(error);
    }
  });
}

try {
  if (!navigator.gpu) throw new Error('This preview needs a browser with WebGPU enabled. Try a current Chrome or Edge on desktop, or run the native demo.');
  // Pin the wasm/js URLs to the build checksum so browsers never run a
  // stale cached build after an update.
  const manifest = await fetch('./manifest.json', { cache: 'no-store' }).then((r) => r.json()).catch(() => null);
  const build = manifest?.['pkg/justdo_command_bg.wasm']?.slice(0, 12) || '';
  const wasm = await import(`./pkg/justdo_command.js?v=${build}`);
  await wasm.default(build ? `./pkg/justdo_command_bg.wasm?v=${build}` : undefined);
  const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)').matches;
  await startPreview(() => {
    if (demo === 'alert') wasm.run_alert(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'alert-dialog') wasm.run_alert_dialog(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'aspect-ratio') wasm.run_aspect_ratio(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'attachment') wasm.run_attachment(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'avatar') wasm.run_avatar(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'badge') wasm.run_badge(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'breadcrumb') wasm.run_breadcrumb(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'bubble') wasm.run_bubble(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'accordion') wasm.run_accordion(params.get('variant') || 'default', dark, reducedMotion);
    else if (demo === 'tabs') wasm.run_tabs(params.get('variant') || 'pill', dark, reducedMotion);
    else if (demo === 'button') wasm.run_button(params.get('variant') || 'variants', dark, reducedMotion);
    else if (demo === 'button-group') wasm.run_button_group(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'calendar') wasm.run_calendar(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'card') wasm.run_card(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'carousel') wasm.run_carousel(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'chart') wasm.run_chart(params.get('variant') || 'bar', dark, reducedMotion);
    else if (demo === 'checkbox') wasm.run_checkbox(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'collapsible') wasm.run_collapsible(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'combobox') wasm.run_combobox(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'command') wasm.run_command(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'context-menu') wasm.run_context_menu(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'data-table') wasm.run_data_table(params.get('variant') || 'basic', dark, reducedMotion);
    else if (demo === 'date-picker') wasm.run_date_picker(params.get('variant') || 'basic', dark, reducedMotion);
    else wasm.run_metallic(params.get('variant') || 'metallic', dark, reducedMotion);
  });
  status.setAttribute('aria-busy', 'false');
  status.dataset.state = 'ready';
  if (reducedMotion) status.remove();
  else {
    status.addEventListener('transitionend', () => status.remove(), { once: true });
    // Also clean up in a background tab where CSS transitions may be skipped.
    setTimeout(() => status.remove(), 200);
  }
} catch (error) {
  console.error(error);
  status.setAttribute('role', 'alert');
  status.setAttribute('aria-busy', 'false');
  status.removeAttribute('aria-label');
  status.replaceChildren();
  const heading = document.createElement('strong');
  heading.textContent = 'Preview could not start';
  const detail = document.createElement('p');
  detail.textContent = error instanceof Error ? error.message : String(error);
  const retry = document.createElement('button');
  retry.textContent = 'Try again';
  retry.onclick = () => location.reload();
  status.append(heading, detail, retry);
}
