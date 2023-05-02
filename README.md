# Projet Cryptographie ISEN 2023

Le projet a 3 composants :
- L'autorité de certification Racine
- L'AE qui est un backend web
- L'AEFront qui est le frontend de l'AE

## ACR
L'ACR est un binaire qui génère les clés et certificat de l'ACR et de l'ACI.

### Génération de l'ACR
```bash
acr root
```

### Génération de l'ACI
```bash
acr intermediate
```
## AE
Pour que l'ae fonction il faut que `ca-root.crt`, `aci.crt` et `aci.key` soit dans un dossier nommé "aci" dans le CWD.

## AEFront
C'est un front en Angular qui permet de faire des requêtes à l'AE.