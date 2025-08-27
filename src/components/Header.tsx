"use client"
import Link from "next/link";
import Nav from "./Nav";
import MobileNav from "./MobileNav";


export default function Header() {
  return (
    <header className="py-8 xl:py-12 text-white">
      <div className="container mx-auto flex justify-between items-center">
        <Link href="/">
        <h1 className="text-3xl font-semibold">
            uttkarzsh<span className="text-accent">.</span>
        </h1>
        </Link>
        <div className="hidden xl:flex items-center gap-8">
            <Nav/>
        </div>

        <div className="xl:hidden">
            <MobileNav></MobileNav>
        </div>
      </div>
    </header>
  );
}