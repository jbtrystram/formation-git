% Formation Git
% Jb Trystram
% 10-08-2021


# Collaborer avec Git

Avec des trains et du rust !


# Introduction

Git : "Magie noire pour voyager dans le temps en ajoutant ou soustrayant des fichiers textes a d'autres fichiers textes"
  
  - Conserver tout l'historique d'évolution d'une base de code.
  - Travailler sur un ou plusieurs changements en parrallèle sans casser le projet.
  - Synchroniser tout ça a plusieurs

-> Objectif d'aujourd'hui : apprendre a travailler a plusieurs sur la meme base de code.

# Point de départ


---

Ce que vous savez déjà faire :


- Cloner 

--- 


 - travailler puis commit

--- 


 - pousser des commits


 - télécharger les commits du repo distant


# Devenons un cheminot 

### le coeur de git
  - les 3 états : commited, staged, unstaged
  - Voies parrallèles : branches
  - Bouger un aiguillage : rebase
  - Construire une gare : tags

### Astuces et bonus
 - ammend et -p
 - bisect
 - reflog
 - stash
 - cherry pick (range of commits)
 - submodules

#### Github: 
- pull requests
- issues 
- (Private vs Pub repo)

## Vocabulaire

branch
remote
main


## Mais où suis-je ?

- log
- status
- show

## Les 3 états

Commited : changements enregistrés dans un commit
staged : changements enregistrés pas encore commités : `git diff --staged`
unstaged : changements pas encore sélectionnés pour un commit : `git diff`
- astuce: `git add -p`

## Branches

Modifier le code sans impacter main.

- Créér une branche : la branche se fait toujours a partir d'un commit

=> todo : faites une branche et ajoutez votre nom au hello world! 


- merge 

### Conflits


La branche ! Faites pull puis essayer de rebase votre branche. 
<!-- TODO  -->

- résoudre le conflit puis commiter. 

### Rebase

## Format

3-4h
vendredi aprem
