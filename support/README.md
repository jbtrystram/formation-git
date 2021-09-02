% Formation Git
% Jb Trystram
% 10-08-2021


# Collaborer avec Git

Avec du rust en bonus !


# Introduction

Git : "Magie noire pour voyager dans le temps en ajoutant ou soustrayant des fichiers textes a d'autres fichiers textes"
  
  - Conserver tout l'historique d'évolution d'une base de code.
  - Travailler sur un ou plusieurs changements en parrallèle sans casser le projet.
  - Synchroniser tout ça a plusieurs

-> Objectif d'aujourd'hui : travailler a plusieurs sur la meme base de code, simplement et sans risques

# Point de départ

Mettons nous d'accords sur ce que vous savez déjà faire.

---
# level : Git sorcerer skills

- Cloner : `git clone`
- Commiter des fichiers après modifications : `git add && git commit`
- Pousser des commits vers un repo distant : `git push`
- Télécharger les commits du repo distant : `git pull`

--- 

# level up : git mage

## le coeur de git
  - les 3 états des fichiers
  - chaines de commits et pointeurs
  
---

### Etats des fichiers

 - commited : changements enregistrés dans un commit
 - staged : changements enregistrés pas encore commités : `git diff --staged`
 - modified: changements pas encore sélectionnés pour un commit : `git diff`
  
```
  Commited  ---work--> modified ---add---> staged \
      ^--------------commit------------------|
```
astuce: `git add -p`

---

### Commits et pointeurs
 
Commit = reférence commit parent + changements (patch) + metadata
tag = un pointeur vers un commit
branche = pointeur vers un commit qui suit chaque nouveau commit
 
schema

---

## Manier git 
  
  - branches: univers parallèles 
  - rebase: voyager dans le temps
  - tags: faire des releases fiables
  
### Branches

Copie cachée du repo

- créer une branche : `git checkout -b turbo` alias `git branch turbo && git checkout turbo`.
la branche se fait toujours a partir d'un commit.
- merger une branche : `git merge main` (sur la branche courante).

=> todo : faites une branche et ajoutez votre nom au hello world! 

### Rebase

- comme merger une branche mais mieux
- refaire l'histoire avec interactif

-> attention à la mise a jour forcée! Préferer `git revert` dans certains cas.

# Level up : git wizzard

## Astuces staging area
 - `add && commit` -> `git commit -a` (ou `-p` pour faire du selectif) 
  -> explorez les options !
  
- bisect : trouver un commit problématique

---

stash : Rapidement changer de contexte.

`git stash push` : changements stagés et non stagés mis de côté.
Astuce : ajoutez un message : `git stash push -m "fixing array oob #334"`
`git stash pop` : retirer et réappliquer la dernière entrée de la stash.
`git stash apply` : la même mais sans supprimer l'entrée dans la stash.
`git stash branch <branch_name>` : créer une branche avec le contenu de la dernière entrée. 
 
 Possible d'avoir plusieurs entrées et de choisir.
 
 ## Mais où suis-je ?
 
 - log 
 - status :
 - show : montrer les détails du dernier commit
 
 ## Astuces git tree 
 - reflog
 - cherry pick (range of commits)
 - submodules

## Github: 
- pull requests
- issues 
- (Private vs Pub repo)

## Resources

- https://git-scm.com/docs (beaucoup de pages sont traduites en français)
- https://www.atlassian.com/fr/git