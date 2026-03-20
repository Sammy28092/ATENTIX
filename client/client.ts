import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// Asegúrate de que el nombre coincida con tu proyecto
import { AdhdLearningPlatform } from "../target/types/adhd_learning_platform"; 

describe("adhd_learning_platform", () => {
  // Configura la conexión a tu entorno local de Solana
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AdhdLearningPlatform as Program<AdhdLearningPlatform>;
  const provider = anchor.getProvider();
  
  // Generamos un usuario "falso" para hacer la prueba
  const userProfile = anchor.web3.Keypair.generate();

  it("¡Inicializa el perfil del usuario con TDAH!", async () => {
    await program.methods.initializeUser()
      .accounts({
        userProfile: userProfile.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userProfile])
      .rpc();
    
    console.log("✅ Perfil creado exitosamente en la blockchain.");
  });

  it("¡Suma puntos después de un juego de seguimiento de instrucciones!", async () => {
    // Simulamos que el usuario ganó 50 puntos
    await program.methods.addScore(50)
      .accounts({
        userProfile: userProfile.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();
      
    console.log("✅ ¡50 puntos añadidos a la cuenta del usuario!");
  });
});
