const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

const template = html`
<style>
:host {
  --bg: var(--surface-2, lightgray);
  --mask-corner-cut-squares: conic-gradient(at 0.8rem 0.8rem,#000 75%,transparent 0) -0.4rem -0.4rem;
}
main {
  background: var(--bg);
  min-height: var(--size-fluid-10);
  -webkit-mask: var(--mask-corner-cut-squares);
}
</style>
<main><slot></slot></main>
`

/**
 * Extensible prompt that accepts custom controls that ease the input of custom data
 */
export class Window extends HTMLElement {
  static TAG = "fido-window";
	static observedAttributes = [];
  
  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed', delagatesFocus: true});
		this.#$root.append(template.content.cloneNode(true))
  }
}
customElements.define(Window.TAG, Window);