import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { BarcodeDetectorPolyfill } from "@undecaf/barcode-detector-polyfill";

import Ad4mConnect, {
  AuthStates,
  ConnectionStates,
  Ad4mConnectOptions,
  ConfigStates,
} from "./core";

import Loading from "./components/Loading";
import Settings from "./components/Settings";
import Start from "./components/Start";
import Disconnected from "./components/Disconnected";
import AgentLocked from "./components/AgentLocked";
import RequestCapability from "./components/RequestCapability";
import VerifyCode from "./components/VerifyCode";
import CouldNotMakeRequest from "./components/CouldNotMakeRequest";
import ScanQRCode from "./components/ScanQRCode";
import Header from "./components/Header";
import autoBind from "auto-bind";
import { getForVersion, removeForVersion, setForVersion } from "./utils";

export { getAd4mClient } from "./utils";

function detectMob() {
  const toMatch = [
    /Android/i,
    /webOS/i,
    /iPhone/i,
    /iPad/i,
    /iPod/i,
    /BlackBerry/i,
    /Windows Phone/i,
  ];

  return toMatch.some((toMatchItem) => {
    return navigator.userAgent.match(toMatchItem);
  });
}

const styles = css`
  :host {
    --primary-color: #fff;
    --heading-color: #fff;
    --body-color: #a7a7a7;
    --success-color: #52d652;
    --background-color: #000;
    --start-color: #a4adff;
    --end-color: #d273ff;
    --gradient: linear-gradient(90deg, var(--start-color), var(--end-color));
  }

  .wrapper {
    font-family: "DM Sans", Helvetica, Arial, sans-serif;
    position: fixed;
    display: grid;
    place-content: center;
    top: 0;
    left: 0;
    color: var(--body-color);
    height: 100vh;
    width: 100vw;
  }

  * {
    box-sizing: border-box;
  }

  .check-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .check-list li {
    display: flex;
    gap: 10px;
    margin-bottom: 10px;
  }

  .items {
    display: flex;
    flex-direction: column;
    flex-gap: 50px;
    gap: 50px;
  }

  .items--small {
    flex-gap: 20px;
    gap: 20px;
  }

  .button {
    text-decoration: none;
    cursor: pointer;
    border: 0;
    color: var(--background-color);
    background: var(--gradient);
    height: 50px;
    font-weight: 600;
    min-width: 100px;
    padding: 0px 30px;
    border-radius: 8px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    font-family: inherit;
    font-size: 15px;
  }

  @media (min-width: 800px) {
    .button {
      min-width: 200px;
      padding: 0px 40px;
      font-size: 17px;
    }
  }

  .heading {
    color: var(--heading-color);
    font-size: 18px;
    font-weight: 600;
    margin: 0;
    margin-bottom: 10px;
  }

  .heading.nomargin {
    margin: 0;
  }

  .body {
    padding: 0;
    margin: 0;
    font-size: 14px;
    line-height: 1.5;
  }

  .buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-gap: 10px;
    gap: 10px;
  }

  .button--full {
    width: 100%;
    display: flex;
  }

  .button--link {
    padding: 0;
    height: auto;
    background: none;
    color: var(--primary-color);
    font-size: inherit;
    min-width: auto;
    text-decoration: none;
  }

  .button--link:hover {
    text-decoration: underline;
  }

  .button--secondary {
    background: var(--background-color);
    border: 1px solid var(--primary-color);
    color: var(--primary-color);
  }

  .dialog {
    background-color: var(--background-color);
    z-index: 10;
    border-radius: 8px;
    width: calc(100vw - 10px);
    max-width: 500px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .dialog__header {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px 30px;
    gap: 10px;
    color: var(--primary-color);
    margin-bottom: 20px;
    font-size: 14px;
  }

  .dialog__logo {
    display: flex;
    text-align: center;
    width: 25px;
  }

  .dialog__content {
    min-height: 300px;
    display: grid;
    place-content: center;
    padding-top: 0;
    padding-left: 30px;
    padding-right: 30px;
    padding-bottom: 30px;
  }

  .dialog__connect {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-gap: 50px;
    gap: 50px;
    position: relative;
  }

  .dialog__logo svg {
    width: 100%;
  }

  .dialog__connect-ad4m {
    width: 80px;
    height: 80px;
    background: var(--background-color);
    padding: 20px;
    box-shadow: 0px 4px 7px 0px rgb(0 0 0 / 8%);
    border-radius: 50%;
    position: relative;
  }

  .dialog__connect-ad4m:before {
    content: "";
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: -1;
    margin: -2px;
    border-radius: inherit;
    background: var(--gradient);
  }

  .dialog__connect-app {
    width: 80px;
    height: 80px;
  }

  .dialog__connect-check:before {
    content: "";
    display: block;
    width: 40px;
    border-bottom: 2px dashed var(--body-color);
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translateY(-50%) translateX(-50%);
  }

  .dialog__connect-check svg {
    position: relative;
  }

  .text-center {
    text-align: center;
  }

  .uppercase {
    text-transform: uppercase;
  }

  .input {
    display: flex;
    flex-direction: column;
    flex-gap: 10px;
    gap: 10px;
  }

  .input__label {
    font-size: 12px;
  }

  .input__field {
    border-radius: 8px;
    outline: 0;
    height: 60px;
    color: var(--heading-color);
    background-color: var(--background-color);
    padding: 0px 30px;
    font-size: 20px;
    border: 1px solid var(--body-color);
  }

  .input__field:focus {
    border: 1px solid var(--primary-color);
    box-shadow: 0px 0px 0px 1px var(--primary-color);
  }

  .ad4mConnect__backdrop {
    position: absolute;
    top: 0;
    left: 0;
    height: 100vh;
    width: 100vw;
    background-color: rgba(0, 0, 0, 0.5);
  }

  .ad4mConnect__locked {
    position: fixed;
    top: 0;
    left: 0;
    background: linear-gradient(
      90deg,
      rgba(2, 0, 36, 1) 0%,
      rgba(38, 3, 23, 1) 41%,
      rgba(51, 4, 31, 1) 100%
    );
    height: 100vh;
    width: 100vw;
    padding: 36px;
    display: flex;
    align-items: center;
    flex-direction: column;
    font-family: "Comfortaa", cursive;
  }

  .lds-ring {
    display: block;
    position: relative;
    width: 80px;
    height: 80px;
    margin: 0 auto;
    margin-top: 24px;
  }
  .lds-ring div {
    box-sizing: border-box;
    display: block;
    position: absolute;
    width: 64px;
    height: 64px;
    margin: 4px;
    border: 4px solid var(--primary-color);
    border-radius: 50%;
    animation: lds-ring 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
    border-color: var(--primary-color) transparent transparent transparent;
  }
  .lds-ring div:nth-child(1) {
    animation-delay: -0.45s;
  }
  .lds-ring div:nth-child(2) {
    animation-delay: -0.3s;
  }
  .lds-ring div:nth-child(3) {
    animation-delay: -0.15s;
  }
  @keyframes lds-ring {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .disconnected {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    padding: 10px 0;
    text-align: center;
    background: red;
  }

  .qr-scanner {
    background: black;
    position: fixed;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    place-content: center;
  }

  .qr-scanner .stop {
    position: absolute;
    z-index: 10;
    left: 50%;
    bottom: 10px;
    transform: translateX(-50%);
  }

  .qr-scanner video {
    height: 100vh;
    width: 100vw;
    object-fit: cover;
  }
`;

@customElement("ad4m-connect")
export class Ad4mConnectElement extends LitElement {
  static styles = [styles];

  @state()
  private _code = null;

  @state()
  private _isMobile = null;

  @state()
  private _hasClickedDownload = null;

  @state()
  private _isRemote = false;

  @state()
  private _client: Ad4mConnect;

  @state()
  private _isOpen: boolean = false;

  @state()
  private uiState:
    | "settings"
    | "start"
    | "qr"
    | "requestcap"
    | "verifycode"
    | "disconnected"
    | "agentlocked" = "start";

  @property({ type: String, reflect: true })
  appName = null;

  @property({ type: String, reflect: true })
  appDesc = null;

  @property({ type: String, reflect: true })
  appDomain = null;

  @property({ type: String, reflect: true })
  appIconPath = null;

  @property({ type: String, reflect: true })
  capabilities = [];

  // TODO: localstorage doesnt work here
  @property({ type: String })
  token = getForVersion("ad4murl") || "";

  // TODO: localstorage doesnt work here
  @property({ type: String, reflect: true })
  port = parseInt(getForVersion("ad4mport")) || 12000;

  // TODO: localstorage doesnt work here
  @property({ type: String, reflect: true })
  url = getForVersion("ad4murl") || "";

  get authState(): AuthStates {
    return this._client.authState;
  }

  get connectionState(): ConnectionStates {
    return this._client.connectionState;
  }

  connectedCallback() {
    super.connectedCallback();
    autoBind(this);

    this._isMobile = detectMob();

    this._client = new Ad4mConnect({
      appName: this.appName,
      appDesc: this.appDesc,
      appDomain: this.appDomain,
      appUrl: window.location.origin,
      appIconPath: this.appIconPath,
      capabilities: Array.isArray(this.capabilities)
        ? this.capabilities
        : JSON.parse(this.capabilities),
      port: this.port || parseInt(getForVersion("ad4mport")) || 12000,
      token: this.token || getForVersion("ad4mtoken"),
      url: this.url || getForVersion("ad4murl"),
    });

    this._client.on("configstatechange", this.handleConfigChange);
    this._client.on("authstatechange", this.handleAuthChange);
    this._client.on("connectionstatechange", this.handleConnectionChange);

    this.loadFont();
  }

  private async unlockAgent(passcode) {
    await this._client.ad4mClient.agent.unlock(passcode);
  }

  private verifyCode(code) {
    this._client.verifyCode(code);
  }

  private changeUrl(url) {
    this._client.setUrl(url);
  }

  private changePort(port: number) {
    this._client.setPort(port);
  }

  private changeUIState(state) {
    this.uiState = state;
  }

  private changeIsRemote(bol: boolean) {
    this._isRemote = bol;
  }

  private changeCode(code) {
    this._code = code;
  }

  private onDownloaded() {
    this._hasClickedDownload = true;
  }

  private handleAuthChange(event: AuthStates) {
    const customEvent = new CustomEvent("authstatechange", {
      detail: event,
    });
    if (event === "locked") {
      this._isOpen = true;
    }
    this.dispatchEvent(customEvent);
    this.requestUpdate();
  }

  private handleConfigChange(name: any, val: string) {
    this[name] = val;
    if (val) {
      setForVersion("ad4m" + name, val);
    } else {
      removeForVersion("ad4m" + name);
    }
    this.requestUpdate();
  }

  private handleConnectionChange(event: ConnectionStates) {
    if (event === "connected") {
      this.changeUIState("requestcap");
    }
    if (event === "disconnected") {
      this._isOpen = true;
    }
    if (event === "not_connected") {
      this.changeUIState("start");
    }
    const customEvent = new CustomEvent("connectionstatechange", {
      detail: event,
    });
    this.dispatchEvent(customEvent);
    this.requestUpdate();
  }

  private loadFont() {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.type = "text/css";
    link.crossOrigin = "anonymous";
    link.href =
      "https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;700&display=swap";
    document.head.appendChild(link);
  }

  private async startCamera(e) {
    try {
      window["BarcodeDetector"].getSupportedFormats();
    } catch {
      window["BarcodeDetector"] = BarcodeDetectorPolyfill;
    }

    await this.changeUIState("qr");

    setTimeout(async () => {
      const video = this.shadowRoot.querySelector("video");

      if (!video) return;

      const media = await navigator.mediaDevices.getUserMedia({
        audio: false,
        video: { facingMode: "environment" },
      });

      video.srcObject = media;
      video.autoplay = true;
    }, 100);
  }

  async connect() {
    this._isOpen = true;
    this.requestUpdate();
    const client = await this._client.connect();
    return client;
  }

  getAd4mClient() {
    return this._client.ad4mClient;
  }

  async connectRemote(url) {
    try {
      const client = await this._client.connect(url);
      this.changeUIState("requestcap");
      return client;
    } catch (e) {
      if (e.message === "Socket closed with event 4500 Invalid Compact JWS") {
        this.changeUIState("requestcap");
      }
    }
  }

  async requestCapability(bool) {
    try {
      await this._client.requestCapability(bool);
      this.changeUIState("verifycode");
    } catch (e) {
      console.warn(e);
    }
  }

  async isAuthenticated() {
    return this._client.checkAuth();
  }

  setOpen(val: boolean) {
    this._isOpen = val;
  }

  renderViews() {
    if (this.connectionState === "connecting") {
      return Loading();
    }

    if (this.uiState === "qr") {
      return ScanQRCode({
        changeState: this.changeUIState,
        onSuccess: (url) => this._client.connect(url),
        uiState: this.uiState,
      });
    }

    if (this.authState === "locked") {
      return AgentLocked({
        unlockAgent: this.unlockAgent,
        reconnect: this.connect,
      });
    }

    if (this.uiState === "settings") {
      return Settings({
        port: this.port,
        changePort: this.changePort,
        isRemote: this._isRemote,
        changeIsRemote: this.changeIsRemote,
        url: this.url,
        changeState: this.changeUIState,
        changeUrl: this.changeUrl,
        connectToPort: this._client.connectToPort,
        connectRemote: this.connectRemote,
      });
    }

    if (this.connectionState === "not_connected") {
      return Start({
        scanQrcode: this.startCamera,
        connect: this.connect,
        isMobile: this._isMobile,
        hasClickedDownload: this._hasClickedDownload,
        onDownloaded: this.onDownloaded,
        changeState: this.changeUIState,
      });
    }

    if (this.connectionState === "connected") {
      if (this.uiState === "verifycode") {
        return VerifyCode({
          code: this._code,
          changeCode: this.changeCode,
          changeState: this.changeUIState,
          verifyCode: this.verifyCode,
        });
      }

      return RequestCapability({
        changeState: this.changeUIState,
        requestCapability: this.requestCapability,
        capabilities: this.capabilities,
        appname: this.appName,
        setOpen: this.setOpen,
        appiconpath: this.appIconPath,
      });
    }

    if (this.connectionState === "disconnected") {
      return Disconnected({
        reconnect: this.connect,
      });
    }

    if (this.connectionState === "port_not_found") {
      return CouldNotMakeRequest();
    }
  }

  render() {
    if (this._isOpen === false) return null;
    if (this.authState === "authenticated") return null;
    return html`
      <div class="wrapper">
        <div class="dialog">
          ${Header()}
          <main class="dialog__content">${this.renderViews()}</main>
        </div>
        <div class="ad4mConnect__backdrop" />
      </div>
    `;
  }
}

export default function Ad4mConnectUI(props: Ad4mConnectOptions) {
  const element = new Ad4mConnectElement();

  Object.entries(props).forEach(([key, value]) => {
    element[key] = value;
  });

  document.body.appendChild(element);

  return element;
}
