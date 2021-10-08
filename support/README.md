---
type: slide
slideOptions:
transition: slide
theme: Black
  
---

# Collaborer avec Git

Avec du rust en bonus !

![](https://github.com/jbtrystram/formation-git/raw/main/support/images/intro.png)

---

# Git ?


> “Magie noire pour voyager dans le temps en ajoutant ou soustrayant des fichiers textes a d'autres fichiers textes”

<span>- Conserver tout l'historique d'évolution d'une base de code.<!-- .element: class="fragment" data-fragment-index="1" --></span>
<span>- Travailler sur un ou plusieurs changements en parrallèle sans casser le projet.<!-- .element: class="fragment" data-fragment-index="2" --></span>
<span>- Synchroniser tout ça a plusieurs<!-- .element: class="fragment" data-fragment-index="3" --></span>
<span>- *Sans risques*<!-- .element: class="fragment" data-fragment-index="4" --></span>

Note:
L'objectif c'est de level up vos skills où que vous en soyez.

---

# lvl 1 : git sorcerer
Terrain connu

- Cloner : `git clone`
- Commiter des fichiers après modifications : `git add && git commit`
- Pousser des commits vers un repo distant : `git push`
- Télécharger les commits du repo distant : `git pull`

---

## le coeur de git
- chaines de commits
- les 3 états des fichiers
- commit

---

### 1 commit en détail

```
commit bbd550e501503a9d580b33b2d867ee3eee133ae8 (origin/main, origin/HEAD)
parent a72c052f5766f82b31f2caaea6a78460714c830c
Author: jbtrystram <jbtrystram@redhat.com>
Date:   Mon Aug 16 16:38:02 2021 +0200

    update changelog

diff --git a/CHANGELOG.md b/CHANGELOG.md
index 3cb9172..63ddc4f 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -6,6 +6,7 @@
  - Added a `set` operation to easily add credentials or a gateway to a device.
  - Devices and apps can now be listed if not ID is specified :  `drg get apps` will list existing apps.
  Plural and singular forms of a resource can be used interchangeably.
+ - Endpoints information in whoami -e. It's also possible to specify a service name to get only the url.

```

Note:
Commit = reférence commit parent + changements (patch) + metadata

---

### États des fichiers

- Untracked: fichiers non suivis par git.
- modified: changements pas encore sélectionnés pour un commit : `git diff`
- staged : changements enregistrés pas encore commités : `git diff --staged`
- commited : changements enregistrés dans un commit.


```
  Commited  ---(do work)--> modified ---(git add)---> staged
      ^--------------(git commit)-----------------------|
```

----


![](https://git-scm.com/book/en/v2/images/lifecycle.png)

---

### Rappels

+ Commitez souvent et petit (atomicité)
+ Ne poussez pas de code qui compile pas, sauf bonne raisons !
+ Les erreurs c'est pas grave: `git revert` est là.


---

### Astuces
+ Staging partiel : `git add -p`
+ combiner `add` et `commit`: `git commit -am`
+ Modifier un commit a postériori : `git commit --amend`

---

# level up : git mage

- branches: univers parallèles
- rebase: voyager dans le temps
- tags: faire des releases fiables


---


### Branche

pointeur vers un commit qui suit chaque nouveau commit

```
A---B---C---D---E---F  main
```

```
A---B---C---D---E---F---G  main
```


---

### Branches


```shell
               X---Y---Z   feature
             / 
    A---B---C---D---E---F  main
```

+ créer une branche : `git checkout -b feature` alias `git branch feature && git checkout feature`.
+ La branche se fait toujours a partir d'un commit.
+ Le pointeur suivra les commits ultérieurs


---

### Merge

```shell
               W---X---Y---Z
             /              \
    A---B---C---D---E---F---WXYZ  main,feature
```
Merger une branche : `git merge feature`.
Attention : le merge se fait vers HEAD.


<span>TP: où est la gare ?<!-- .element: class="fragment" data-fragment-index="1" --></span>

Note:
Faites une branche et ajoutez une phrase pour demandez ou est la gare, puis merge!

---

```shell
                          X feature
                         / 
    A---B---C---D---E---F  main
```
Fast-forward merge:
```shell
    A---B---C---D---E---F---X  main,feature
```

---

Note : la branche se met a jour en fonction de votre checkout:
```shell
               W---X---Y---Z   O---P feature
             /              \ /
    A---B---C---D---E---F---WXYZ main
```


---

### Rebase

- Rejouer l'historique d'une branche à partir d'un autre commit :

```shell
               X---Y---Z  feature
             / 
    A---B---C---D---E---F  main
```
`git rebase main feature` :
```shell
                           X---Y---Z  feature
                         / 
    A---B---C---D---E---F  main
```

Note:
On peut rebase sur un commit directement.
Vu que une branche n'est jamais qu'un pointeur vers un commit.


---

#### Refaire l'histoire avec rebase
```shell
               X---Y---Z  feature
             / 
    A---B---C---D---E---F  main
```

`git rebase -i <parent_du_dernier_commit_a_modifier>`

```shell
               X  feature
             / 
    A---B---C---D---E---F  main
```

Note:
`git rebase -i C # ou HEAD~3`

---

Possiblités :

- réordonner les commits
- Supprimer des commits
- changer le message de commit
- altérer le commit (ajouter des choses ou le diviser)
- fusionner des commits (squash ou fixup)

---

Astuces :

`HEAD~3` ou `main~10`

`git commit --amend` == `git rebase -i HEAD^`

=> Attention à la mise a jour forcée!
=> Experimentez sur la branche `prez/mess`

Note:
Préferer `git revert` si la branche est déjà poussée.
  
--- 

Mais où suis-je ?

- `git log` (`--graph --all`)
- `git status`
- `git show` : montrer les détails d'un commit


---

### Tag

Un tag est une simple étiquette vers un commit:
`git tag -a v2.1 -m "release sept 21" <commitRef>`

```
git show v2.1

tag v2.1
Tagger: jbtrystram <jbtrystram@redhat.com>
Date:   Thu Sep 2 16:30:17 2021 +0200

release sept 21

commit aa2692440781883f49cb2127fd2b0272fe7f7a38 (HEAD -> main, tag: v2.1)
Merge: 47351eb 66c8c5a
Author: jbtrystram <jbtrystram@redhat.com>
Date:   Thu Sep 2 16:14:02 2021 +0200

    Merge branch 'feature/command'
```

---

# Level up : git wizzard

+ bisect : trouver un commit problématique
  `git bisect <dernier_bon_commit>`

=> essayez sur la branche `broken`

---

## stash : Rapidement changer de contexte.

- `git stash push` : changements stagés et non stagés mis de côté. \
  Astuce : ajoutez un message : `git stash push -m "fixing array oob #334"`
- `git stash pop` : retirer et réappliquer la dernière entrée de la stash.

---

- `git stash apply` : comme pop mais sans supprimer l'entrée dans la stash.
- `git stash branch <branch_name>` : créer une branche avec le contenu de la dernière entrée.

Note:
Possible d'avoir plusieurs entrées et de choisir.


---

## Manipuler l'historique avec reflog

> "Oh la boulette !"

Historique de tous les changements du pointeur HEAD.

---

```shell
    A---B---C---D main
```

drop du commit D avec rebase:
```
git rebase -i HEAD~2
    A---B---C  main
```

`git reflog`
```shell
0b3a33c (HEAD -> main) HEAD@{0}: rebase (finish): returning to refs/heads/main
0b3a33c (HEAD -> main) HEAD@{1}: rebase (start): checkout HEAD~2
899c5a9 HEAD@{2}: commit: D
0b3a33c HEAD@{3}: commit: C
111723a HEAD@{4}: commit: B
2233e43 HEAD@{5}: commit (initial): A
```

---

- `git reset 899c5a9`: les changements entre 899c5a9 et le résultat du rebase est restaurés dans le répertoire de travail (unstaged).
- Pour restaurer le repo a l'état voulu : `git reset --hard 899c5a9` -> le commit D est restauré.

- Rien n'est jamais perdu avec git (même si rebase avec des commits droppés).


Experimentez !

---

## Cherry-picking

Recopier un commit sur la branche courrante :

`git cherry-pick <refCommit>`

- On peut aussi utiliser une série de commit contigus :
  `git cherry pick <refPremierCommit>^..<refDernierCommit>`


Note:
Ici on spécifie le commit parent du premier commit, car le range commence après. Parfois git est étrange!

---

## Github:
- pull requests
- issues
- (Private vs Pub repo)

---

## Idées

- submodules: repo dans un repo
- hooks : voir dans `.git/hooks` Les hooks sont locaux.
- conventionnal commits : https://www.conventionalcommits.org/en/v1.0.0/

---

## Resources

- `man git`
- https://git-scm.com/docs (beaucoup de pages sont traduites en français)
- https://www.atlassian.com/fr/git
