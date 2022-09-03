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
:host(:focus) {
  border: var(--border-size-1) solid var(--indigo-3);
}
#prompt {
  display: flex;
  background: var(--bg);
  box-sizing: border-box;
  padding: var(--size-fluid-1) var(--size-fluid-2);
  -webkit-mask: var(--mask-corner-cut-squares);
}
#text-entry {
  flex: 10;
  min-height: var(--size-fluid-3);
  font-size: var(--font-size-fluid-1);
  font-family: monospace;
  text-align: start;
  outline: none;
}
</style>
<div id="prompt">
  <div id="text-entry" contenteditable></div>
</div>
`;

/**
 * Extensible prompt that accepts custom controls that ease the input of custom data
 */
export class Prompt extends HTMLElement {
  static TAG = "fido-prompt";
	static observedAttributes = [];
	static formAssociated = true;

  #$root;
  #internals;
  
  constructor() {
    super();
    
		this.#$root = this.attachShadow({ mode: 'closed', delagatesFocus: true});
		this.#$root.append(template.content.cloneNode(true))

		if ('ElementInternals' in window && 
			'setFormValue' in window.ElementInternals.prototype) {
			this.#internals = this.attachInternals();
			this.#internals.setFormValue(this.value);
		}
  }


	// form associated element
	get value() { return '' }
	get form() { return this.#internals.form; }
	get name() { return this.getAttribute('name'); }
	get type() { return this.localName; }
	get validity() { return this.#internals.validity; }
	get validationMessage() { return this.#internals.validationMessage; }
	get willValidate() { return this.#internals.willValidate; }
	checkValidity() { return this.#internals.checkValidity(); }
	reportValidity() { return this.#internals.reportValidity(); }
}
customElements.define(Prompt.TAG, Prompt);
