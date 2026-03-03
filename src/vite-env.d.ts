/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_CLICKUP_OPENAI_TOKEN?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
