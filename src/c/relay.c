//
//  Use this file to import your target's public headers that you would like to expose to Swift.
//

#import "absd.h"
#import <mach/mach.h>
#import <sys/sysctl.h>
#import <stdio.h>
#import <stdlib.h>
#import <CoreFoundation/CoreFoundation.h>

extern kern_return_t bootstrap_look_up(mach_port_t bp, const char *service_name, mach_port_t *sp);

mach_port_t ABSD_PORT = MACH_PORT_NULL;

uint32_t NAC_MAGIC = 0x50936603;

int nac_init(const void *certificate_bytes, size_t certificate_len, uint64_t *out_ctx, void **out_session_request, size_t *session_requestCnt) {
    if (ABSD_PORT == MACH_PORT_NULL) {
        kern_return_t kret = bootstrap_look_up(bootstrap_port, "com.apple.absd", &ABSD_PORT);
        if (kret != KERN_SUCCESS) {
            printf("bootstrap_look_up failed\n");
            return kret;
        }
    }
    
    // endianness? what's that?
    int ret = rawNACInit(ABSD_PORT, NAC_MAGIC, (vm_offset_t)certificate_bytes, certificate_len, out_ctx, (vm_offset_t *)out_session_request, (mach_msg_type_number_t *)session_requestCnt);
    if (ret != 0) {
        printf("remoteNACInit failed: %d\n", ret);
        return ret;
    }
    printf("done\n");

    return 0;
}

int nac_key_establishment(uint64_t val_ctx, const void *session_response, size_t session_response_len) {
    return rawNACKeyEstablishment(ABSD_PORT, NAC_MAGIC, val_ctx, (vm_offset_t)session_response, session_response_len);
}

int nac_sign(uint64_t val_ctx, const void* data, size_t data_len, void **out_signature, size_t* out_sig_len) {
    int ret = rawNACSign(ABSD_PORT, NAC_MAGIC, val_ctx, (vm_offset_t)data, data_len, (vm_offset_t *)out_signature, (mach_msg_type_number_t *)out_sig_len);
    if (ret != 0) {
        printf("remoteNACSign failed: %d\n", ret);
        return ret;
    }
    return 0;
}

extern CFTypeRef MGCopyAnswer(CFStringRef property);

char* mg_copy_answer(const char* firstProperty) {
    CFStringRef property = CFStringCreateWithCString(kCFAllocatorDefault, firstProperty, kCFStringEncodingUTF8);
    CFTypeRef answer = MGCopyAnswer(property);
    CFRelease(property);
    
    size_t malloc_size = 128;
    char *buildNumberBuf = calloc(1, malloc_size);
    CFStringGetCString(answer, buildNumberBuf, malloc_size, kCFStringEncodingUTF8);
    CFRelease(answer);
    return buildNumberBuf;
}
