type ContainerProps = {
  children: React.ReactNode;
};

export const Container = ({ children }: ContainerProps) => {
  return (
    <div className="text-lg container bg-stone-50 border-indigo-500 border m-auto p-5 rounded max-w-1xl mx-auto text-gray-800 space-y-6 px-4">
      {children}
    </div>
  );
};
