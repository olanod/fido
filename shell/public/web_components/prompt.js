const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

const template = html`
<style>
:host {
  --bg: var(--surface-2, lightgray);
}
#prompt {
  display: flex;
  background: var(--bg);
  box-sizing: border-box;
  padding: var(--size-fluid-1) var(--size-fluid-2);
}
#prompt textarea {
  flex: 10;
  border: none;
  background: none;
  outline: none;
  resize: none;
  min-height: var(--size-fluid-3);
  font-size: var(--font-size-fluid-1);
}
</style>
<div id="prompt">
  <textarea></textarea>
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
