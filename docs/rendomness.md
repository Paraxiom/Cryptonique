For a Proof of Concept (POC), the primary objective is often to demonstrate the feasibility of the concept and to identify any major issues that could prevent it from being developed into a full-fledged system. In many cases, a simplified model or simulation is adequate for this purpose. However, the level of rigor you apply to your POC depends on your specific goals and requirements.

If you aim to rigorously validate the security and robustness of your cryptographic system through methods like brute-force attacks, then yes, the quality of your random number generation becomes critically important, even at the POC stage. In such cases, you might want to consider more advanced forms of randomness and entropy beyond just PRNGs.

Here's how the three approaches you mentioned could fit into a more rigorous POC:

    QRNG: Could provide the highest quality of randomness but might be costly or require partnerships.

    Quantum Computing Resources: Could offer a middle-ground solution with better-than-classical randomness and the potential to scale into a quantum solution in the future.

    Entropy Sources: Could offer a DIY approach that's better than simple PRNGs but requires careful implementation and validation.

Each of these would allow you to run more convincing and rigorous tests, such as brute-force attacks, to validate your POC's robustness. If you're looking to eventually commercialize this product or use it in a high-security environment, then the extra rigor at the POC stage could provide valuable insights and save you time and effort in the long run.

 Quantum Random Number Generator (QRNG)

    Specialized Hardware: QRNGs typically use quantum phenomena like photon polarization to generate truly random numbers. This would indeed require specialized hardware, either purchased or accessed through a service.

    Partnerships: Some companies and research institutions provide QRNG as a service, either through APIs or specialized hardware. Partnering with such providers (like ID Quantique, QuintessenceLabs, etc.) could be an option.

2. Actual Quantum Computing Resources

    Cloud Access: Both IBM Q and D-Wave offer cloud-based quantum computing resources that you can access through their APIs. You don't necessarily need a partnership for basic access, although for extensive or commercial use, a partnership might be beneficial.

    Libraries: These providers offer SDKs and libraries that include functions for generating quantum random numbers, among other quantum operations.

3. Entropy Sources

    System Entropy: Operating systems often collect entropy from various hardware events like keyboard presses, mouse movements, and disk activity. This can be accessed through specific system calls.

    Mouse Movements: Using mouse movements to supplement entropy is a commonly used technique, especially in cryptographic applications. It's considered a form of a hardware random number generator, although care must be taken to ensure that the mouse movements are processed in a way that maximizes their entropy.

    External Sensors: Sensors like atmospheric noise sensors, Geiger counters, or even webcam data can also be used to generate high-entropy data.

Partnerships vs DIY

    D-Wave/IBM: Partnering could provide you with more resources and possibly better rates, but for limited usage, their free tiers might be sufficient.

    Mouse Movements/Sensors: These would be a DIY solution and would involve you capturing this data yourself to feed into your system. This approach doesn't require any partnerships but does require careful implementation to ensure high-quality entropy.