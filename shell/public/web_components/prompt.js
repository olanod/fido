const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

const promptTpl = html`
<style>
:host {
  --bg: var(--surface-2);
}
:host(:focus) fido-box { --box-active: 100%; }
#wrap { padding: 0.5em var(--font-size-fluid-0); }
#text-entry {
  color: white;
  flex: 10;
  min-height: var(--font-size-fluid-1);
  line-height: var(--font-size-fluid-1);
  font-size: var(--font-size-1);
  font-family: monospace;
  text-align: start;
  outline: none;
  margin: 0;
  padding: 0.5em var(--font-size-fluid-0);
}
</style>
<div id="wrap">
  <fido-box>
    <slot></slot>
    <pre id="text-entry" contenteditable><br></pre>
  </fido-box>
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
		this.#$root.append(promptTpl.content.cloneNode(true));
    this.#$entry = this.#$root.getElementById('text-entry');

		if ('ElementInternals' in window && 
			'setFormValue' in window.ElementInternals.prototype) {
			this.#internals = this.attachInternals();
			this.#internals.setFormValue(this.value);
		}
  }
  
  connectedCallback() {
    this.#$root.addEventListener('keypress', this.#enterSubmitOrNewLine);
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
    this.#$entry.innerHTML = '\n';
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

const boxTpl = html`
<style>
:host {
  --box-border: var(--brand);
  --box-border-active: var(--violet-3);
  --box-active: 0%;
  --clip: var(--pixel-corners, none);
}
#border {
  clip-path: var(--clip);
  background: linear-gradient(to right, var(--box-border) 40%, var(--box-border-active) 60%);
  background-size: 240% 100%;
  background-position: var(--box-active);
  transition: background-position 500ms;
  padding: 2px;
}
#content {
  background: var(--bg);
  clip-path: var(--clip);
  display: flex;
}
</style>
<div id="border">
  <div id="content">
    <slot></slot>
  </div>
</div>
`

/**
 * A box with a pixelated border
 */
export class Box extends HTMLElement {
  static TAG = 'fido-box';
	static observedAttributes = [];

  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed'});
		this.#$root.append(boxTpl.content.cloneNode(true))
  }
}
customElements.define(Box.TAG, Box);
