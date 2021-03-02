adc $5, %al
adc $10, %ah
adc %al, %ah
adc %eax, %eax
adc $10, %ax
adc %sil, %dil
adc %rax, %rsi
adc $0xFFFF, %rax
adc %ebx, %eax

adc al, 5
adc ax, bx
adc eax, ebx
adc rax, rbx