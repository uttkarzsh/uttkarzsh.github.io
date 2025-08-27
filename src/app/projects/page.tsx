import {projects} from "./../../../public/projects/projects"

export default function Projects() {


  return (
    <section className="min-h-screen px-4 sm:px-6 lg:px-12 py-12 text-white">
      <h1 className="text-3xl sm:text-4xl font-bold mb-10 text-center">Projects</h1>
      <div className="flex flex-col gap-10 max-w-5xl mx-auto">
        {projects.map((project, index) => (
          <div
            key={index}
            className="bg-black/40 border border-white/10 rounded-2xl shadow-lg overflow-hidden hover:scale-[1.02] transition-transform duration-300"
          >
            <img
              src={project.image}
              alt={project.title}
              className="w-full h-48 sm:h-64 md:h-72 object-cover"
            />
            <div className="p-4 sm:p-6">
              <h2 className="text-xl sm:text-2xl font-semibold mb-2">{project.title}</h2>
              <p className="text-gray-400 text-sm sm:text-base mb-4">{project.description}</p>
              <div className="flex flex-wrap gap-2">
                {project.tech.map((t, i) => (
                  <span
                    key={i}
                    className="px-2 sm:px-3 py-1 text-xs sm:text-sm bg-blue-600/20 border border-blue-500 rounded-full"
                  >
                    {t}
                  </span>
                ))}
              </div>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}