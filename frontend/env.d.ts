/// <reference types="vite/client" />

declare interface Window {
  __SSR_STATE__: Record<string, any>;
}