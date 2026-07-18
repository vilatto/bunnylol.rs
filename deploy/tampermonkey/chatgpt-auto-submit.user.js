// ==UserScript==
// @name         bunnylol ChatGPT Auto-Submit
// @namespace    bunnylol.rs
// @version      1.0
// @description  Auto-submits the ChatGPT composer when the page loads with a ?q= URL parameter (e.g. from bunnylol's "chatgpt" command). OpenAI's ?q= param still pre-fills the composer natively but no longer auto-sends it (patched July 2025 as an anti prompt-injection measure), so this just clicks Send once the composer has content.
// @match        https://chatgpt.com/*
// @grant        none
// @run-at       document-idle
// ==/UserScript==

(function () {
  "use strict";

  // Only act when we arrived via a ?q= link (e.g. bunnylol) — never auto-submit
  // during normal chatgpt.com usage.
  const params = new URLSearchParams(window.location.search);
  if (!params.has("q")) return;

  const maxAttempts = 100; // ~10s at 100ms, enough for the composer to hydrate
  let attempts = 0;

  const interval = setInterval(() => {
    attempts++;
    const sendButton = document.getElementById("composer-submit-button");

    // The button exists but stays disabled until the composer actually has
    // text in it, so waiting for "enabled" also confirms the ?q= prefill landed.
    if (sendButton && !sendButton.disabled) {
      sendButton.click();
      clearInterval(interval);
    } else if (attempts >= maxAttempts) {
      clearInterval(interval);
    }
  }, 100);
})();
