#include <stdio.h>

typedef enum {
    links,
    rechts,
} rotatie

typedef struct {
    rotatie soort;
    int aantal_beweging;
} notatie

int rotatie100(int plaats, notatie beweging)
{
    int beweging_v;
    switch beweging.soort{
    case(links):
        beweging_v = -1*beweging.aantal_beweging;
        break;
    case(rechts):
        beweging_v = beweging.aantal_beweging;
        break;
    }
    return (plaats + beweging_v) %100;
}

void main (void)
{
    notatie bewegingen[] = {{1, 10}};
    int lengte = sizeof(beweging)/sizeof(beweging[0]);
    int plaats;
    int resultaat = 0;
    for(int i=0; i<lengte; i++)
    {
        plaats = rotatie100(plaats, bewegingen[i]);
        if(plaats == 0)
        {
            resultaat++;
        }
    }
    printf("resulaat = %i", resulaat);
}