/// <reference types="vite/client" />

declare interface Window {
  __SSR_STATE__: Record<string, any>;
}

declare module '*.vue' {
  import { ComponentOptions } from 'vue'
  const componentOptions: ComponentOptions
  export default componentOptions
}