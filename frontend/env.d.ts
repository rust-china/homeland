/// <reference types="vite/client" />

declare interface Window {
  __SSR_STATE__: Record<string, any>;
}

import type { App as VueApp, VNode } from 'vue'
declare module 'vue' {
  interface App extends VueApp {
    render: (vnode: VNode | null, container: Element) => void
  }
}

declare module '*.vue' {
  import { ComponentOptions } from 'vue'
  const componentOptions: ComponentOptions
  export default componentOptions
}