/* The controller library (libpad) the game calls: port states, mode
 * queries and the actuator (vibration) set-up. */
#ifndef PSX_LIBPAD_H
#define PSX_LIBPAD_H

#ifndef NULL
#define NULL 0
#endif

/* PadGetState results. */
#define PadStateDiscon   0
#define PadStateFindPad  1
#define PadStateFindCTP1 2
#define PadStateFindCTP2 3
#define PadStateReqInfo  4
#define PadStateExecCmd  5
#define PadStateStable   6

/* PadInfoMode terms. */
#define InfoModeCurID   1
#define InfoModeCurExID 2

void PadInitDirect(unsigned char*, unsigned char*);
void PadStartCom(void);
int PadGetState(int);
int PadInfoMode(int, int, int);
int PadSetActAlign(int, unsigned char*);
int PadSetMainMode(int socket, int offs, int lock);
void PadSetAct(int, unsigned char*, int);

#endif
