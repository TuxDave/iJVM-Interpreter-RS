Documentazione per la corretta interpretazione dei file compilati iJVM per mic1

# .iJVM
file compilati per l'interprete iJVM scritto in MAL (Mic1 Assembly Language)
I tipi in gioco sono solo interi, ma all'interno dell'implementazioni troviamo:
- Variabili, Costanti:  32 bit
- Valori:               8 bit
- Offset:               8/16 bit

## Header
Il file è interpretabile in hex e presenta parole composte da 4 byte
Ogni istruzione del file ha una lunghezza variabile da 1 a 4 byte
I byte di header conferiscono informazioni su come è organizzato il programma compilato nel file

Il file presenta:
- 4 bytes di magic constant per riconoscere che è un file ijvm, come CAFEBABE in java (ignore)
- 4 bytes che danno infomazioni di dove si trova la constant pool nella memoria di mic1 (ignore)
- 4 bytes che indicano quanti bytes occupano le costanti (1 costante = 4 bytes), se è definita 1 costante segnerà 4 ecc 
- 4*n bytes sono di fila tutte le n costanti scritte in big endian (prima i byte più significativi)
- 4 bytes a 0 che separano dall'inizio del codice eseguibile

## Executable
La sezione che comprende il codice eseguibile è composta da istruzioni.
Ogni istruzione ha l'OPCODE che identifica l'operazione, seguito dai suoi eventuali parametri, fino ad arrivare a 4 bytes nelle WIDE.
Leggendo il file:
- 4 bytes che indicano il numero di bytes che compongono il codice eseguibile (metodi compresi)
- di seguito tutto il codice che compone il metodo main
- di seguito tutti il codice che compone gli altri metodi

È presente in memoria uno spazio che contiene i parametri dei metodi e le variabli locali per ogni metodo (e ogni chiamata di una funzione ne ha uno associato).
Questo spazio nell'implementazione originale sarebbe nello stack, per comodità verrà effettuato in uno stack a parte per linearità

### Metodi
Gli altri metodi, a differenza del <i>main<i>, prima del listato delle istruzioni hanno 4 bytes che indicano:
- i primi 2 il numero di parametri
- 2 secondi 2 il numero di variabili locali
  (ovviamente le costanti sono condivise da tutti i metodi (e non modificabili)).