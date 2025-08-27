import { Button } from "@/components/ui/button";
import Socials from "@/components/Socials";
import Photo from "@/components/Photo";

export default function Home() {
  return <section className="h-full">
      <div className="container mx-auto h-full">
        <div className="flex flex-col xl:flex-row items-center justify-between xl:pt-8 xl:pb-24">
          <div className="text-center xl:text-left order-2 xl:order-none">
            <h1 className="text-2xl">Hi. This is</h1>
            <span className="text-6xl mb-1 mt-1">Uttkarsh,</span>
            <br/><h1 className="text-xl">A Web3 Developer & ZK Enthusiast.</h1>
            <p className="mt-4 text-sm">Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolore odit velit voluptas amet, temporibus rerum.</p>
            <div className="flex flex-col xl:flex-row items-center gap-8 mt-6">
            <div className="mb-8 xl:mb-0"><Socials containerStyles="flex gap-6" iconStyles="w-9 h-9  border border-accent rounded-full flex justify-center items-center text-accent text-base hover:bg-accent hover:text-primary hover:transition-all duration-500"/></div>
            </div>
          </div>
          <div className="order-1 xl:order-none mb-8 xl:mb-8"><Photo/></div>
        </div>
      </div>
    </section>
}
