const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

const template = html`
<style>
:host {
  --bg: var(--surface-2);
  --border: var(--brand);
  --border-focus: var(--violet-3);
  --clip: var(--pixel-corners, none);
}

#wrap {
  padding: var(--font-size-fluid-0);
}
#border {
  clip-path: var(--clip);
  background: linear-gradient(to right, var(--border) 40%, var(--border-focus) 60%);
  background-size: 240% 100%;
  background-position: 0%;
  transition: background-position 500ms;
  padding: 2px;
}
:host(:focus) #border { background-position: 100%; }
#prompt {
  display: flex;
  background: var(--bg);
  box-sizing: border-box;
  padding: var(--font-size-fluid-0) var(--font-size-fluid-1);
  clip-path: var(--clip);
  filter: drop-shadow(0 0 1px var(--brand));
  color: white;
}
#text-entry {
  flex: 10;
  min-height: var(--font-size-fluid-1);
  line-height: var(--font-size-fluid-1);
  font-size: var(--font-size-1);
  font-family: monospace;
  text-align: start;
  outline: none;
  margin: 0;
}
</style>
<div id="wrap">
  <div id="border">
    <div id="prompt">
      <slot></slot>
      <pre id="text-entry" contenteditable><br></pre>
    </div>
  </div>
</div>
`;

/**
 * Extensible prompt that accepts custom controls that ease the input of custom data
 */
export class Prompt extends HTMLElement {
  static TAG = 'fido-prompt';
	static observedAttributes = [];
	static formAssociated = true;

  #$root;
  #$entry;
  #internals;
  
  constructor() {
    super();
    
		this.#$root = this.attachShadow({ mode: 'closed', delagatesFocus: true});
		this.#$root.append(template.content.cloneNode(true))
    this.#$entry = this.#$root.getElementById('text-entry');

		if ('ElementInternals' in window && 
			'setFormValue' in window.ElementInternals.prototype) {
			this.#internals = this.attachInternals();
			this.#internals.setFormValue(this.value);
		}
  }
  
  connectedCallback() {
    this.#$root.addEventListener('keypress', this.#enterSubmitOrNewLine)
  }
  
  #enterSubmitOrNewLine = e => {
    if (e.code == 'Enter') {
      e.preventDefault();
      if (e.shiftKey) {
        this.#addNewLine();
        getSelection().collapse(this.#$entry.lastChild);
      } else {
        let val = this.value;
        if (!val) return;
        this.#internals?.setFormValue(val);
        this.form?.dispatchEvent(new SubmitEvent('submit', {submitter: this}));
        this.form?.reset();
      }
    }
  }
  #addNewLine = () => this.#$entry.append(document.createTextNode('\n'));

  formResetCallback() {
    this.#$entry.innerHTML = '<br>';
  }

	// form associated element
	get value() { return this.#$entry.textContent }
	get form() { return this.#internals?.form; }
	get name() { return this.getAttribute('name'); }
	get type() { return this.localName; }
	get validity() { return this.#internals?.validity; }
	get validationMessage() { return this.#internals?.validationMessage; }
	get willValidate() { return this.#internals?.willValidate; }
	checkValidity() { return this.#internals?.checkValidity(); }
	reportValidity() { return this.#internals?.reportValidity(); }
}
customElements.define(Prompt.TAG, Prompt);
