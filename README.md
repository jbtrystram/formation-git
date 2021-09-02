# Formation Git : support

# Application Rust

L'application <name> ici sert uniquement a démontrer et a expérimenter différentes fonctionnalités de git.

Pour compiler et lancer: 
```shell
cargo run 
```

# Slides 

Les slides sont écrites en markdown dans le readme du dossier `support`.
Pour un slide deck HTML avec pandoc : 

```shell
pandoc -t revealjs -V revealjs-url=https://unpkg.com/reveal.js@3.9.2/ -s support/README.md -o support/slides.html 
```