# Signature du code (Windows)

L'installateur Windows `Fadera-Installateur-<version>.exe` est construit par GitHub Actions (workflow « Publication d'une version »).
Tant que la signature n'est pas accordée, Windows affiche « Éditeur inconnu » (Microsoft Defender SmartScreen) : choisir
*Informations complémentaires → Exécuter quand même*. Les sommes SHA-256 de la page de la version permettent de vérifier le fichier.

## Politique de signature (Code Signing Policy)

- **Ce qui est signé** : l'installateur Windows publié sur les [versions GitHub](../../../releases). Rien d'autre.
- **D'où il vient** : uniquement de la compilation automatique du dépôt public, à partir de l'étiquette `vX.Y.Z`. Aucun fichier compilé sur un poste personnel n'est signé.
- **Qui décide** : Faderaseny (auteur, relecteur et approbateur). La clé privée reste chez le service de signature ; elle n'est jamais dans le dépôt.
- **Vie privée** : l'installateur et `fadera.exe` n'envoient aucune donnée personnelle. Le programme n'utilise le réseau que si un programme Fadéra le demande (fonctions `http_*`, paquets).
- **Contact** : ouvrir un ticket sur https://github.com/Faderaseny/fadera/issues.

## Mise en place avec SignPath (gratuit pour l'open source)

1. Demander l'accès gratuit : formulaire « Open Source » de la SignPath Foundation (signpath.org). Conditions : licence reconnue par l'OSI (Fadéra : MIT), dépôt public, téléchargement gratuit, aucun comportement malveillant.
2. Une fois accepté : créer l'organisation et le projet SignPath (`fadera`), ajouter le « Trusted Build System » GitHub.com et créer la politique de signature `release-signing`.
3. Configuration d'artefact du projet (le workflow envoie un `.zip` contenant l'installateur) :

   ```xml
   <artifact-configuration xmlns="http://signpath.io/artifact-configuration/v1">
     <zip-file>
       <pe-file path="A-SIGNER-Fadera-Installateur-*.exe">
         <authenticode-sign/>
       </pe-file>
     </zip-file>
   </artifact-configuration>
   ```

4. Dans GitHub : *Settings → Secrets and variables → Actions* :
   - secret `SIGNPATH_API_TOKEN` (jeton d'API d'un utilisateur CI SignPath) ;
   - variable `SIGNPATH_ORGANIZATION_ID` (identifiant de l'organisation) ;
   - facultatif : variables `SIGNPATH_PROJECT_SLUG` (défaut `fadera`) et `SIGNPATH_POLICY_SLUG` (défaut `release-signing`).
5. Dès que `SIGNPATH_ORGANIZATION_ID` existe, la prochaine publication envoie l'installateur à SignPath et publie la version signée.
   Sans cette variable, rien ne change : l'installateur est publié non signé.
6. **Après acceptation seulement**, ajouter au README la mention exigée par SignPath :
   « Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org). »

Remarques : le certificat est au nom de la SignPath Foundation (c'est elle qui apparaît comme éditeur). Une signature enlève « Éditeur inconnu »,
mais SmartScreen peut demander quelques téléchargements avant de ne plus afficher d'avertissement pour une nouvelle version.
