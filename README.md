# OBS-LIKE-Rust
caputre I/O


# Visualisation Module

## C'est quoi ?

Un programme qui **capture** simultanément 5 choses en continu :

1. **L'écran** - Ce qui s'affiche à l'écran
2. **L'audio** - Ce qu'on entend (le microphone)
3. **Les actions** - Les clics souris et touches clavier
4. **Le Bluetooth** - Les données Bluetooth
5. **Internet/Ethernet** - Les paquets réseau

## Ensuite ?

Après avoir capturé ces 5 choses, le programme :

1. **Compresse** les données pour qu'elles prennent moins de place
2. **Envoie** tout à une pool externe

## À quoi ça sert ?

- Enregistrer une session pour la rejouer après
- Surveiller ce qu'on fait sur l'ordinateur
- Faire du streaming (retransmettre l'écran en direct)
- Analyser les patterns d'utilisation

## Configuration

Un fichier de configuration est nécessaire pour paramétrer le programme :
- La qualité des screenshots (1-100)
- La fréquence (combien de fois par seconde)
- L'adresse IP de la pool
- L'activation/désactivation de chaque capteur

## C'est tout

Le projet fait juste ça : **capture → compresse → envoie**.
