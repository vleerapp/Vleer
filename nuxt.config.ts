// https://nuxt.com/docs/api/configuration/nuxt-config
import { defineNuxtConfig } from "nuxt/config";

export default defineNuxtConfig({
  modules: ["@nuxt/devtools"],
  devtools: {
    enabled: true,
  },
  ssr: false,
});
