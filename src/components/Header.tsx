"use client"
import Link from "next/link";
import { Button } from "./ui/button";
import Nav from "./Nav";
import { usePathname } from "next/navigation";


export default function Header() {
    const pathname = usePathname();
  return (
    <header className="py-8 xl:py-12 text-white bg-pink-50/20">
      <div className="container mx-auto flex justify-between items-center">
        <Link href="/">
        <h1 className="text-4xl font-semibold">
            uttkarzsh<span className="text-accent">.</span>
        </h1>
        </Link>
        <div className="hidden xl:flex items-center gap-8">
            <Nav/>
            <Link href="/contact">
                { !(pathname === "/contact") &&
                   <Button>Hire Me</Button> 
                }
            </Link>
        </div>
      </div>
    </header>
  );
}