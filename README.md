# split-include

Découpe un `autoconf.h` (style Kconfig) en plusieurs formats : assembleur,
C, Perl et Python.

```sh
split-include [-v] <dossier>
```

L’outil lit `<dossier>/autoconf.h` et écrit dans le même dossier :

| Fichier                 | Contenu                                          |
|-------------------------|--------------------------------------------------|
| `autoconf.py`           | `NOM = valeur` (`False` pour un `#undef`)        |
| `autoconf.pl`           | `$NOM = valeur;` (`0` pour un `#undef`)          |
| `config.<groupe>.h`     | `#define` / `#undef`, un fichier par groupe      |
| `config.<groupe>.inc`   | `NOM EQU valeur` (`0` pour un `#undef`)          |

## Réglages

Seules les lignes `#define CONFIG_<groupe>_<nom> <valeur>` et
`#undef CONFIG_<groupe>_<nom>` sont lues. Le groupe est ce qui précède le
premier `_` ; `<nom>` est écrit sans le préfixe `CONFIG_<groupe>_`.

Ces réglages sont ignorés avec un avertissement :

- sans préfixe de groupe (`CONFIG_SMP`) ;
- sans valeur (`#define CONFIG_A_B`) ;
- dont le groupe ou le nom n’est pas un identifiant valide.

`autoconf.py` et `autoconf.pl` n’ont qu’un seul espace de noms : si deux
groupes ont le même nom une fois le préfixe retiré, un avertissement est
émis et la dernière valeur l’emporte.

## Modèles de `config.<groupe>.h`

Pour un groupe, l’outil cherche `config.<groupe>.h` (groupe en minuscules)
dans les dossiers de la variable `CONFIG_TEMPLATES` (liste de chemins), puis
dans `<dossier>/../templates`. Sans modèle, il génère un fichier avec une
garde `AUTOCONF_<groupe>_H`.

Un modèle est recopié tel quel, sauf la ligne `// end` : les réglages sont
insérés à cet endroit. Un modèle sans ligne `// end` est une erreur.

```c
#ifndef TPL_H
// end
#endif /* TPL_H */
```
