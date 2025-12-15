import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { strongholdStorage } from "@/lib/stronghold";
import { generateKeypair, registerKeypair } from "@/commands/identify";

interface SecretState {
  keypair: number[] | null;
  deviceId: string | null;
  _hasHydrated: boolean;
  setHasHydrated: (state: boolean) => void;
  init: () => Promise<void>;
}

export const useSecretStore = create(
  persist<SecretState>(
    (set, get) => ({
      keypair: null,
      deviceId: null,
      _hasHydrated: false,
      setHasHydrated: (state) => set({ _hasHydrated: state }),
      async init() {
        const { keypair } = get();
        if (!keypair) {
          const keypair = await generateKeypair();
          const deviceId = await registerKeypair(keypair);
          set({ keypair, deviceId });
        } else {
          const deviceId = await registerKeypair(keypair);
          set({ deviceId });
        }
      },
    }),
    {
      name: "secret-store",
      storage: createJSONStorage(() => strongholdStorage),
    }
  )
);

useSecretStore.persist.onFinishHydration((state) => {
  state.init();
  state.setHasHydrated(true);
});
