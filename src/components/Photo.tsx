"use client";

import { motion } from "framer-motion";
import Image from "next/image";


const Photo = () => {
  return (
    <div className="w-full h-full relative">
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1, transition: { delay: 0.5, duration: 0.4, ease: "easeIn" } }}
      >
        <div className="relative w-[298px] h-[298px] xl:w-[398px] xl:h-[398px]">
          <Image
            src="/assets/photo.png"
            alt="profile photo"
            priority
            fill
            sizes="(max-width: 1280px) 298px, 398px"
            className="object-cover rounded-full"
          />
        </div>
      </motion.div>
    </div>
  )
}

export default Photo