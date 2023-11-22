//hook for vTaskStartScheduler()FreeRTOSConfig.h, and the xPortGetFreeHeapSize()
#include "riscv-virt.h"


void hook_vTaskStartScheduler(void) 
{
    //print that successfully hooked
    vSendString("hooked vTaskStartScheduler\n");
    vTaskStartScheduler();
}