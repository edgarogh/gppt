# G<sup>P</sup>P<sub>T</sub>

_Generate stupid on-the-fly slideshow presentations (with fake but somewhat credible information) from a prompt, using Ch\*tGPT_

## Generative AI isn't cool

The project was made as a one-time joke with friends, but I don't endorse nor encourage the use of GenAI (Ch\*tGPT, M\*djourney, Stable D\*ffusion...) for a plethora of reason (ecological, ethical, social & more). Have fun looking at the code, but don't do GenAI, kids.

## Limitations

  * Only supports French for the time being, but there's literally just one word to change in the prompt to fix that.
  * Not tested against prompt injection or real HTML/JS XSS. Be careful.

## Usage

Have Rust+cargo installed.

```bash
OPENAI_KEY=0123456789 cargo run
```

The project is built with Rocket, so you can configure it with [a `Rocket.toml`](https://rocket.rs/v0.5-rc/guide/configuration/#rockettoml).
