const p = new DOMParser();
const html = (ss, ...parts) => p.parseFromString('<template>' + parts
	.reduce((t, val, i) => `${t}${strings[i]}${val}`, '')
	.concat(ss[parts.length]) + '</template>', 'text/html').querySelector('template');

/**
 * A box with a pixelated frame/border
 */
export class Frame extends HTMLElement {
  static TAG = 'fido-frame';
	static observedAttributes = [];
  static template = html`
<style>
:host {
  --frame-color: var(--brand);
  --frame-color-active: var(--alt);
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
  height: calc(var(--frame-size, auto) / var(--frame-ratio, 1));
  width: var(--frame-size);

  clip-path: var(--clip);
  background: linear-gradient(var(--frame-active-angle), var(--frame-color) 40%, var(--frame-color-active) 60%);
  background-size: 100% 240%;
  background-position: 0 var(--frame-active);
  box-sizing: border-box;
  transition: background-position 250ms;
  padding: var(--frame);
}
:host([selected]) { --frame-active: 100%; padding: calc(var(--frame) + 1px); }
:host(.s) { --frame-size: var(--size-4); }
:host(.m) { --frame-size: var(--size-5); }
:host(.l) { --frame-size: var(--size-6); }
:host(.box) { --frame-ratio: var(--ratio-square); }
:host(.card) { --frame-ratio: calc(var(--ratio-golden)); }
@media only screen and (min-device-width: 768px){
  :host(.s) { --frame-size: var(--size-5); }
  :host(.m) { --frame-size: var(--size-6); }
  :host(.l) { --frame-size: var(--size-7); }
}
#content {
  color: var(--text-1);
  background: var(--frame-bg);
  clip-path: var(--clip);
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: var(--padding);
  margin: auto 0;
}
</style>
<div id="content" part="content">
  <slot></slot>
</div>
`;

  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed'});
		this.#$root.append(this.constructor.template.content.cloneNode(true))
  }
}
customElements.define(Frame.TAG, Frame);

/**
 * Extensible prompt that accepts custom controls that ease the input of custom data
 */
export class Status extends HTMLElement {
  static TAG = "fido-status";
	static observedAttributes = [];
  static template = html`
<style>
:host { --color: #eef; }
::slotted(*) {
  all: initial;
  color: var(--color);
  font-family: monospace;
  font-size: var(--font-size-0);
  letter-spacing: 1px;
  line-height: var(--font-size-1);
  text-align: start;
  user-select: none;
}
@media (prefers-color-scheme: dark) { :host { --color: #dde; } }
main {
  display: flex;
  height: var(--font-size-1);
  padding: 2px var(--size-1);
}
button {
  all: initial;
  color: var(--color);
  height: var(--font-size-1);
  margin-inline-start: auto;
  padding: 0 var(--size-1);
  letter-spacing: 1.5px;
  user-select: none;
}
button:active {
  font-weight: bold;
  letter-spacing: 1px;
}
</style>
<main>
  <slot></slot>
  <button>···</button>
</main>
`;
  
  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed', delagatesFocus: true});
		this.#$root.append(this.constructor.template.content.cloneNode(true))
  }
}
customElements.define(Status.TAG, Status)

/**
 * Extensible prompt that accepts custom controls that ease the input of custom data
 */
export class Prompt extends HTMLElement {
  static TAG = 'fido-prompt';
	static observedAttributes = [];
	static formAssociated = true;
  static template = html`
<style>
:host { --font-size: 1rem; }
:host(:focus) fido-frame { --frame-active: 100%; }
fido-frame { --frame-bg: var(--surface-2); --frame-color: var(--surface-1); }
#wrap { padding: var(--size-1); }
#text-entry {
  flex: 10;
  min-height: var(--font-size-1);
  line-height: var(--font-size-1);
  font-size: var(--font-size);
  font-family: monospace;
  text-align: start;
  outline: none;
  margin: 0;
  padding: clamp(0.8em, 1vw, 1rem) var(--font-size-0);
}
</style>
<div id="wrap">
  <fido-frame>
    <slot></slot>
    <pre id="text-entry" contenteditable><br></pre>
  </fido-frame>
</div>
`;

  #$root;
  #$entry;
  #internals;
  
  constructor() {
    super();
    
		this.#$root = this.attachShadow({ mode: 'closed', delagatesFocus: true});
		this.#$root.append(this.constructor.template.content.cloneNode(true));
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

/**
 * Grid that can be sorted and have elements selected
 */
export class Grid extends HTMLElement {
  static TAG = 'fido-grid';
	static observedAttributes = [];
  static template = html`
<style>
:host {
  align-items: start;
  display: grid;
  justify-content: center;
  justify-items: center;
  gap: var(--size-2);
  grid-template-columns: repeat(auto-fill, var(--size-6));
}
::slotted(h4) {
  display: flex;
  border-bottom: 1px solid var(--surface-2);
  color: var(--text-2);
  cursor: pointer;
  font-family: monospace;
  grid-column: 1 / -1;
  height: 1.4rem;
  text-align: start;
  margin: 0 var(--font-size-0);
  width: 100%;
}
::slotted(h4:active) { color: var(--brand); border-bottom: 1px solid var(--brand); }
::slotted(h4)::after {
  content: '<';
  display: inline-block;
  margin-inline-start: auto;
  transform: rotate(-90deg);
  transform-origin: center;
  transition: transform 180ms;
}
::slotted(h4.collapse)::after {
  transform: rotate(0deg);
}
::slotted(.collapse:not(h4)) {
  display: none !important;
}

@media only screen and (min-device-width: 768px){
  :host {
    grid-template-columns: repeat(auto-fill, var(--size-7));
    gap: var(--size-3);
  }
}
</style>
<slot></slot>
`;

  #$root;

  constructor() {
    super();
		this.#$root = this.attachShadow({ mode: 'closed'});
		this.#$root.append(this.constructor.template.content.cloneNode(true));
  }

  connectedCallback() {
    this.addEventListener('click', ({target: h4}) => {
      if (!h4.matches('h4')) return;
      const isShowing = h4.classList.contains('collapse');
      h4.classList.toggle('collapse');

      const keyframes = isShowing ?
        [{ transform: 'translateY(-70%) scale(0.4)', opacity: 0 }, { transform: 'translateY(0) scale(1)', opacity: 1 }] :
        [{ transform: 'translateY(-70%) scale(0.4)', opacity: 0 }];
      for (let ele of this.#elementsOf(h4)) {
        if (isShowing) ele.classList.remove('collapse');
        ele.animate(keyframes, { duration: 200, easing: isShowing ? 'ease-out' : 'ease-in' })
          .finished.then(() => { if(!isShowing) ele.classList.add('collapse') });
      }
    })
  }

  *#elementsOf(title) {
    let next = title.nextElementSibling;
    if (next?.matches('h4') || !next) return;
    yield next;
    yield* this.#elementsOf(next);
  }
}
customElements.define(Grid.TAG, Grid);
