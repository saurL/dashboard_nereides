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
git clone https://github.com/votre-repo/nereides-dashboard.git
cd nereides-dashboard
```

2. **Instalez les dépendances**
`npm install`

3. ** Lancez le projet en mode développement :**
```
npm run dev
```

