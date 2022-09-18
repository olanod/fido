const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

const template = html`
<style>
:host {
  --bg: var(--surface-2);
  margin: 1rem auto;
}
main {
  background: var(--bg);
  color: white;
  height: var(--font-size-fluid-1);
  line-height: var(--font-size-fluid-1);
  font-size: var(--font-size-fluid-0);
  font-family: monospace;
  letter-spacing: 1px;
  text-align: start;
  padding: 0 var(--font-size-fluid-0);
}
</style>
<main><slot></slot></main>
`

/**
 * Extensible prompt that accepts custom controls that ease the input of custom data
 */
export class Status extends HTMLElement {
  static TAG = "fido-status";
	static observedAttributes = [];
  
  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed', delagatesFocus: true});
		this.#$root.append(template.content.cloneNode(true))
  }
}
customElements.define(Status.TAG, Status);