"use client";
import Socials from "@/components/Socials";
import Photo from "@/components/Photo";
import { motion } from "framer-motion";
import { FaArrowRight } from "react-icons/fa";

export default function Home() {
  return (
    <section className="h-full text-white">
      {/* Hero Section */}
      <div className="container mx-auto h-full">
        <div className="flex flex-col xl:flex-row items-center justify-between xl:pt-12 xl:pb-24">
          {/* Left Content */}
          <div className="text-center xl:text-left order-2 xl:order-none max-w-xl">
            <h1 className="text-lg sm:text-2xl mb-2 text-gray-300">Hi<span className="text-accent">,</span> this is</h1>
            <h2 className="text-5xl sm:text-6xl font-bold mb-2">Uttkarsh<span className="text-accent">...</span></h2>
            <h3 className="text-lg sm:text-xl mb-4 text-gray-400">
              A Web3 Developer & ZK Enthusiast<span className="text-accent">.</span>
            </h3>
            <p className="text-sm sm:text-base text-gray-400 leading-relaxed">
              I&apos;m passionate about building scalable applications, exploring blockchain ecosystems, 
              and experimenting with cutting-edge technologies in Web3 and the tech world. 
              Always eager to learn, solve complex problems, and contribute to impactful projects.
            </p>

            {/* Social Links */}
            <div className="flex flex-col xl:flex-row items-center gap-6 mt-6">
              <Socials
                containerStyles="flex gap-6"
                iconStyles="w-10 h-10 border border-accent rounded-full flex justify-center items-center text-accent text-lg hover:bg-accent hover:text-primary transition-all duration-500"
              />
            </div>

            {/* Extra Links */}
            <div className="flex flex-col items-center xl:items-start gap-3 mt-8 text-accent text-sm sm:text-base font-medium">
              <a href="/projects" className="flex items-center gap-2 hover:translate-x-1 transition-transform duration-300">
                View Projects <span className="text-accent">→</span>
              </a>
              <a href="/blogs" className="flex items-center gap-2 hover:translate-x-1 transition-transform duration-300">
                Read Blogs <span className="text-accent">→</span>
              </a>
            </div>
          </div>

          {/* Right Photo */}
          <motion.div initial={{ opacity: 0 }}
          animate={{ opacity: 1, transition: { delay: 0.8, duration: 0.4, ease: "easeIn" } }}
          className="relative">
          <div className="order-1 xl:order-none mb-8 xl:mb-0">
            <Photo />
          </div></motion.div>
        </div>
      </div>

      {/* About Me Section */}
      <div className="bg-black/40 border-t border-white/10 py-16 mt-8">
        <div className="container mx-auto max-w-5xl px-6">
          <h2 className="text-3xl sm:text-4xl font-semibold text-center mb-6">About Me</h2>
          <p className="text-gray-400 text-base sm:text-lg leading-relaxed text-center mb-10">
            I am a sophomore at Indian Institute of Technology, Roorkee with a strong interest in{" "}
            <span className="text-accent">Blockchain Development, Zero Knowledge Systems and Smart Contract Security</span>.  
            My journey started with C++ and Web 2.0, and has since expanded into exploring 
            modern frameworks, decentralized technologies, and data-driven problem-solving.  
            I love blending creativity with logic to build projects that are both functional and impactful.
          </p>

          {/* Skills */}
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4 text-center">
            {[
              "Solidity",
              "Foundry",
              "viem/WAGMI",
              "Noir",
              "JavaScript",
              "TypeScript",
              "MERN",
              "TailwindCSS",
              "DSA",
              "Python",
            ].map((skill, i) => (
              <div
                key={i}
                className="px-4 py-3 border border-white/10 rounded-lg bg-black/30 text-gray-300 hover:border-accent hover:text-accent transition"
              >
                {skill}
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}