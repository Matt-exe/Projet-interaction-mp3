L'objectif de ce mini-projet est de créer une application en ligne commande pour gérer une collection de fichiers multimédia.

L'utilisateur saisira une commande suivie d'arguments.
Les commandes à implémenter sont les suivantes :

* `scan` : analyser récursivement un répertoire pour collecter les fichiers supportés
** l'analyse doit extraire les métadonnées du fichier
* `search` : effectue une recherche sur les données gérées (format de la requête ?)
* `write2md` : génèrer un fichier _Markdown_ contenant le résultat d'une requête
* `write2playlist` : générer des playlists, ...

Les données analysées peuvent être sauvegardées au format JSON pour une réutilisation ultérieure.


== Matteo Rabache 22001282

== Structure du projet
Le projet initial est géré avec `cargo` et est structuré en plusieurs modules et fichiers.

. Quelles bibliothèques externes sont utilisées dans le projet ? Expliquez en une phrase leur utilité et donnez l'URL de leur documentation
+
.Répondre ici
....
- structopt : permet d'analyser les arguments de la ligne de commande.
https://docs.rs/structopt/latest/structopt/

- walkdir : constructeur qui créer un itérateur pour parcourir récursivement le répertoire.
https://docs.rs/walkdir/latest/walkdir/

- id3 : permet de lire et écrire des métadonnées de type ID3.
https://docs.rs/id3/latest/id3/

- serde : permet de sérialiser/déserialiser les structures de données rust. 
https://docs.rs/serde/0.9.0-rc2/serde/index.html

- serde_json : permet d'exploiter des données stockées/à stocker au format json.
https://docs.rs/serde_json/latest/serde_json/

- markdown-gen : permet de formater des fichiers markdown et d'y inscrire des données.
https://doc.rust-lang.org/stable/nightly-rustc/rustdoc/html/markdown/index.html

- pls : permet d'analyser et d'écrire au format de liste de lecture PL (playlist).
https://docs.rs/pls/latest/pls/


== Structure de données initiale pour les fichiers audio
Seuls les fichiers audio seront pris en compte.

. Trouvez et documentez dans le code source l'ébauche d'API pour gérer les fichiers audio

== Implémentation des fonctionnalités
Les fonctionnalités ci-dessous sont à intégrer obligatoirement dans le projet.
L'usage de bibliothèques externes pour l'implémentation est non seulement autorisé mais même encouragé.

=== Analyse d'un répertoire (`scan`)
La fonction `scan` doit analyser récursivement un répertoire pour collecter les fichiers supportés.
L'analyse doit extraire les métadonnées du fichier.
Les données analysées peuvent être sauvegardées par exemple au format JSON pour une réutilisation ultérieure.

=== Recherche dans les métadonnées des fichiers (`search`)
La fonction `search` effectue une recherche sur les données gérées et retourne l'ensemble des médias correspondant.
Le format de la requête doit permettre d'interroger les différents champs des fichiers médias.

=== Générer une liste au format Markdown (`write2md`)
La fonction `write2md` (obligatoire) permet de génèrer un fichier _Markdown_ contenant le résultat d'une requête.

=== Générer une playlist (`write2???`)
La fonction `write2???` génère une playlist à partir du résultat d'une requête destiné à un logiciel externe comme https://www.videolan.org/vlc/index.fr.html[vlc].