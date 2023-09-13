((globalThis) => {
    const core = Deno.core;

    globalThis.JWT = {
        generateJwt: async (issuer, audience, expiration_time, capabilities) => {
            return core.opAsync("generate_jwt", issuer, audience, expiration_time, capabilities);
        },
        verifyJwt: async (token) => {
            return core.opAsync("verify_jwt", token);
        }
    };
  })(globalThis);
  