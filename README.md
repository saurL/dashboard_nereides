# Néréides Dashboard

## Description

Néréides Dashboard est une application de tableau de bord développée avec **Tauri** pour le backend et **Vue.js** pour le frontend.  
Elle permet de recevoir et d'afficher des données en temps réel grâce à un système d'événements performant.  

---

## Structure du Projet

- **`src-tauri/`** : Contient tout le code backend écrit en Rust. Cette partie gère la réception des données via une connexion TCP sur `localhost:8080`.  
- **`src/`** : Contient tout le code frontend en Vue.js et TypeScript pour l'interface utilisateur.  

---

## Fonctionnalités

### Réception des Données

Pour récupérer les données, l'application écoute les connexions TCP sur l'adresse `localhost:8080`.  

1. **Structure du paquet :**  
   - Avant de recevoir le contenu des données, le backend lit un **VarInt** indiquant la taille du paquet.  
   - Une fois la taille obtenue, le contenu du paquet est traité sous la forme d'un objet JSON suivant ce format :  
     ```json
     {
       "data": "nom de la donnée",
       "value": "valeur de la donnée"
     }
     ```


### Affichage des Données
La gestion des données reçues repose sur un système d'événements entre le backend et le frontend.

1. **Gestion des événements par Tauri :**
Les données sont transmises sous forme d'événements depuis le backend.

2. **Écoute des événements en TypeScript :**
Le frontend écoute les événements via le module @tauri-apps/api/event :
```typescript
import { listen } from "@tauri-apps/api/event";

listen("data_name", (event) => {
  console.log(event); // Affiche les données reçues
  displayed_data.value = event.payload as number; // Met à jour l'interface
});
```

### Installation et Lancement
1. **Clonez le dépôt :**

```bash
git clone https://github.com/saurL/dashboard_nereides
cd nereides-dashboard
```

2. **Instalez les dépendances**
`npm install`

3. ** Lancez le projet en mode développement :**
```
npm run dev
```
## Déploiement et Mise à Jour sur le Raspberry Pi

### 1. **Compilation automatique sur GitHub Actions**

Lorsque des modifications sont poussées sur la branche `release`, une action GitHub se déclenche automatiquement pour compiler le programme pour l'architecture ARM (`arm64`). Vous pouvez suivre l'avancement de cette action sur l'onglet **Actions** de votre dépôt GitHub.

### 2. **Récupération des fichiers `.deb` ARM64**

Une fois l'action GitHub terminée avec succès, le fichier `.deb` pour architecture ARM64 sera généré et disponible dans les artefacts de l'action. 

Pour vérifier que l'action a bien été exécutée et que le fichier a bien été mis à jour, vous pouvez consulter les logs de l'action dans GitHub. Vous y trouverez un lien pour télécharger le fichier `.deb`.

### 3. **Transfert du fichier `.deb` sur le Raspberry Pi**

Lorsque le fichier `.deb` est prêt, vous devez le transférer vers votre Raspberry Pi en utilisant `scp` (Secure Copy Protocol). Suivez ces étapes :

1. **Téléchargez le fichier `.deb` depuis GitHub**.
2. Ouvrez un terminal sur votre ordinateur local et exécutez la commande suivante pour transférer le fichier sur le Raspberry Pi :

   ```bash
   scp chemin/vers/le/fichier_arm64.deb pi@adresse_ip_du_pi:/home/pi/
   ```
Remplacez `chemin/vers/le/fichier_arm64.deb` par le chemin du fichier téléchargé et `adresse_ip_du_pi` par l'adresse IP de votre Raspberry Pi. /!\ Vous devrez certainement être connecté au même wifi
### 4. Installation du fichier `.deb` sur le Raspberry Pi
Une fois le fichier transféré, connectez-vous à votre Raspberry Pi et installez le paquet `.deb` :
   ```bash
sudo dpkg -i /home/pi/fichier_arm64.deb
   ```
### 5. Mise à jour
À chaque nouveau push sur la branche `release`, le processus ci-dessus sera réexécuté automatiquement, ce qui vous permettra de toujours déployer la version la plus récente de votre programme sur le Raspberry Pi. Il suffit de répéter l'étape 3 (transfert via `scp`) et 4 (installation via `dpkg`).





