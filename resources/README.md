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
Non è ancora noto come sapere il numero di variabili del main, ma si può ovviare con vettore dinamico che cresce quando serve lasciando dei valori nulli in mezzo nei "buchi".
Leggendo il file:
- 4 bytes che indicano il numero di bytes che compongono il codice eseguibile (metodi compresi)
- di seguito tutto il codice che compone il metodo main
- di seguito tutti il codice che compone gli altri metodi

È presente in memoria uno spazio che contiene i parametri dei metodi e le variabli locali per ogni metodo (e ogni chiamata di una funzione ne ha uno associato).
Questo spazio nell'implementazione originale sarebbe nello stack, per comodità verrà effettuato in uno stack a parte per linearità

### Istruzioni
Ogni istruzione come già detto è identificata dal suo OPCODE e riceve eventuali parametri
Ogni comando ha come scoope di visibilità anche lo stack della memoria, nello specifico il TOS e può pushare o poppare.
Ogni comando prende 0, 1 o 2 parametri per la lunghezza massima di OPCODE + PARAMS di 4 bytes
I parametri possono essere di grandezza -> tipo:
- 0 bytes: 
  - NOPARAM: non ci sono parametri da passare, lavora sullo stack
- 1 byte:
  - BYTE: un valore segnato/non segnato hardcoded nel codice
  - VARNUM: un valore non segnato che indica un offset dalla base della lista di variabili locali al metodo
- 2 bytes:
  - OFFSET: un valore segnato usato es nei GOTO che indica un valore di istruzioni sa saltare in su o in giù
  - CONST: un valore non segnato che indica quale costante selezionare dal pool
- 2 bytes special:
  - VARNUM_CONST: 2 bytes che vanno interpretati singolarmente come 2 parametri (WIDE non applicabile):
    - 1°: VARNUM
    - 2°: BYTE
      
In generale è possibile usare WIDE, istruzione che allunga i VARNUM a 2 bytes

Le istruzioni vengono eseguite una di seguito all'altra salvo che non si presentino salti.
I salti permettono di variare l'ordine di esecuzione come si fa con le selezioni ed iterazioni.
Per questo motivo, prima di iniziare l'esecuzione è necessario storare l'intero codice operativo all'interno di una struttura indexable.
Si sfrutterà un vero e proprio Program Counter per tenere traccia dell'istruzione da eseguire.
Preferisco avere una Method Area più simile all'originale (che include OPCODE, p1, p2 ecc) piuttosto di una che includa un oggetto Istr con all'interno i parametri, questo facilità anche i salti che non devono essere ri calcolati
Per permettere l'esecuzione sequenziale e non sequenziale in caso di salti, ogni istruzione effettuerà autonomamente il corretto incremento al PC (sia per gestire il consumarsi dei parametri, sia per gestire i salti);
quindi es: iadd incrementa di 1, bipush di 2, iinc di 3, goto +-x di +-x.
La lettura del codice operativo, seppur debba essere completa, sarà LAZY, quindi verranno caricate in memoria le istruzioni eseguite e quella in esecuzione, se si salta in avanti verrà caricato fino a quella necessasia, per evitare <i>lunghi</i> tempi di caricamento dei programmi

### Metodi
Gli altri metodi, a differenza del <i>main</i>, prima del listato delle istruzioni hanno 4 bytes che indicano:
- i primi 2 il numero di parametri
- 2 secondi 2 il numero di variabili locali (ignorable se usiamo il metodi di incremento dinamico del main)

ovviamente le costanti sono condivise da tutti i metodi (e non modificabili).
