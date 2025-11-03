"use client";

import {FaGithub, FaLinkedin} from "react-icons/fa"
import {RiTwitterXFill} from "react-icons/ri"
import { CiMail } from "react-icons/ci";
import { motion } from "framer-motion";
import Image from "next/image";

export default function Contact() {
  return (
    <section className="min-h-screen px-4 sm:px-6 lg:px-12 py-16 text-white">

      <div className="w-full flex flex-col xl:flex-row items-center xl:items-start justify-center gap-12 xl:gap-24">
  
        {/* Left Column - Socials */}
        <div className="flex flex-col items-start text-left">
          <h1 className="mb-6 text-4xl font-semibold text-accent">My Socials</h1>

          <div className="flex flex-col gap-4">
            {/* GitHub */}
            <a
              href="https://github.com/uttkarzsh"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-3 group"
            >
              <div className="p-3 border border-white/20 rounded-full group-hover:border-accent group-hover:text-accent transition">
                <FaGithub className="text-xl" />
              </div>
              <p className="text-gray-300 group-hover:text-accent transition">
                @uttkarzsh
              </p>
            </a>

            {/* LinkedIn */}
            <a
              href="https://linkedin.com/in/uttkarsh-pawan"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-3 group"
            >
              <div className="p-3 border border-white/20 rounded-full group-hover:border-accent group-hover:text-accent transition">
                <FaLinkedin className="text-xl" />
              </div>
              <p className="text-gray-300 group-hover:text-accent transition">
                Uttkarsh Pawan
              </p>
            </a>

            {/* X (Twitter) */}
            <a
              href="https://x.com/0xuttkarzsh"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-3 group"
            >
              <div className="p-3 border border-white/20 rounded-full group-hover:border-accent group-hover:text-accent transition">
                <RiTwitterXFill className="text-xl" />
              </div>
              <p className="text-gray-300 group-hover:text-accent transition">
                @0xuttkarzsh
              </p>
            </a>

            {/* Mail */}
            <a
              href="mailto:utkbest@gmail.com"
              className="flex items-center gap-3 group"
            >
              <div className="p-3 border border-white/20 rounded-full group-hover:border-accent group-hover:text-accent transition">
                <CiMail className="text-xl" />
              </div>
              <p className="text-gray-300 group-hover:text-accent transition">
                utkbest@gmail.com
              </p>
            </a>
          </div>
        </div>

        {/* Right Column - Image */}
       <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1, transition: { delay: 0.8, duration: 0.4, ease: "easeIn" } }}
          className="relative"
        >
          <div className="relative w-[198px] h-[198px] xl:w-[298px] xl:h-[298px]">
            <Image
              src="/assets/photo2.png"
              alt="imagehoto"
              priority
              fill
              sizes="(max-width: 1280px) 198px, 298px"
              className="object-contain rounded-full"
            />
          </div>
        </motion.div>
      </div>

      <h1 className="text-center text-4xl mb-6 mt-28 font-semibold">Let&apos;s Talk</h1>
      <form
        action="https://formspree.io/f/xvgvvpql"
        method="POST"
        className="max-w-2xl mx-auto bg-black/40 border border-white/10 rounded-2xl shadow-lg p-8 space-y-6"
      >
        <p className="text-gray-300">
          Drop me a message whether you want to <span className="text-accent">hire me</span>, 
          collaborate on a project, or just say hi.
        </p>

        {/* Name */}
        <div>
          <label className="block text-sm mb-2 text-gray-400">Name</label>
          <input
            type="text"
            name="name"
            required
            className="w-full px-4 py-2 rounded-lg bg-black/60 border border-white/20 text-white focus:outline-none focus:border-accent"
          />
        </div>

        {/* Email */}
        <div>
          <label className="block text-sm mb-2 text-gray-400">Email</label>
          <input
            type="email"
            name="email"
            required
            className="w-full px-4 py-2 rounded-lg bg-black/60 border border-white/20 text-white focus:outline-none focus:border-accent"
          />
        </div>

        {/* Message */}
        <div>
          <label className="block text-sm mb-2 text-gray-400">Message</label>
          <textarea
            name="message"
            rows={5}
            required
            className="w-full px-4 py-2 rounded-lg bg-black/60 border border-white/20 text-white focus:outline-none focus:border-accent"
          ></textarea>
        </div>

        {/* Button */}
        <button
          type="submit"
          className="w-full sm:w-auto px-6 py-2 rounded-lg border border-accent text-accent hover:bg-blue-600/20 transition font-medium"
        >
          Send Message
        </button>
      </form>
    </section>
  );
}