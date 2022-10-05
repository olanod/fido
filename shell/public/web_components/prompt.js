const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

const promptTpl = html`
<style>
:host(:focus) fido-frame { --frame-active: 100%; }
fido-frame { --frame-bg: var(--surface-2); }
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
  padding: 0.6em var(--font-size-fluid-0);
}
</style>
<div id="wrap">
  <fido-frame>
    <slot></slot>
    <pre id="text-entry" contenteditable><br></pre>
  </fido-frame>
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

const frameTpl = html`
<style>
:host {
  --frame-color: var(--brand);
  --frame-color-active: var(--violet-3);
  --frame-active: 0%;
  --frame-active-angle: 180deg;
  --frame-bg: var(--surface-1, white);
  --frame: 2px;
  --frame-size: auto;
  --frame-ratio: 1;
  --clip: polygon(
    0px 4px, 2px 4px, 2px 2px, 4px 2px, 4px 0px,
    calc(100% - 4px) 0px, calc(100% - 4px) 2px, calc(100% - 2px) 2px, calc(100% - 2px) 4px, 100% 4px,
    100% calc(100% - 4px), calc(100% - 2px) calc(100% - 4px), calc(100% - 2px) calc(100% - 2px), calc(100% - 4px) calc(100% - 2px), calc(100% - 4px) 100%,
    4px 100%, 4px calc(100% - 2px), 2px calc(100% - 2px), 2px calc(100% - 4px), 0px calc(100% - 4px)
  );
  --padding: 0;
  display: block;
  height: var(--frame-size);
  width: calc(var(--frame-size, auto) * var(--frame-ratio, 1));

  clip-path: var(--clip);
  background: linear-gradient(var(--frame-active-angle), var(--frame-color) 40%, var(--frame-color-active) 60%);
  background-size: 100% 240%;
  background-position: 0 var(--frame-active);
  box-sizing: border-box;
  transition: background-position 250ms;
  padding: var(--frame);
}
:host(.box) {
  --frame-size: var(--size-fluid-5);
  --frame-ratio: var(--ratio-square);
}
:host(.card) {
  --frame-size: var(--size-fluid-6);
  --frame-ratio: calc(var(--ratio-golden));
}
@media only screen and (min-device-width: 768px){
  :host(.box) { --frame-size: var(--size-fluid-6); }
  :host(.card) { --frame-size: var(--size-fluid-7); }
}
#content {
  background: var(--frame-bg);
  clip-path: var(--clip);
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: var(--padding);
  margin: auto 0;
}
</style>
<div id="content">
  <slot></slot>
</div>
`;

/**
 * A box with a pixelated frame/border
 */
export class Frame extends HTMLElement {
  static TAG = 'fido-frame';
	static observedAttributes = [];

  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed'});
		this.#$root.append(frameTpl.content.cloneNode(true))
  }
}
customElements.define(Frame.TAG, Frame);

const gridTpl = html`
<style>
:host {
  align-items: start;
  display: grid;
  justify-content: center;
  justify-items: center;
  gap: var(--size-fluid-3);
  grid-template-columns: repeat(auto-fill, var(--size-fluid-6));
}
@media only screen and (min-device-width: 768px){
  :host {
    grid-template-columns: repeat(auto-fill, var(--size-fluid-7));
    gap: var(--size-fluid-4);
  }
}
</style>
<slot></slot>
`;

/**
 * Grid that can be sorted and have elements selected
 */
export class Grid extends HTMLElement {
  static TAG = 'fido-grid';
	static observedAttributes = [];

  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed'});
		this.#$root.append(gridTpl.content.cloneNode(true))
  }
}
customElements.define(Grid.TAG, Grid);
