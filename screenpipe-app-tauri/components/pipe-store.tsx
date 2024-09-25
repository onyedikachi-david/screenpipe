import React, { useEffect, useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Card } from "@/components/ui/card";
import { MemoizedReactMarkdown } from "@/components/markdown";
import { CodeBlock } from "@/components/ui/codeblock";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import { useSettings } from "@/lib/hooks/use-settings";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "./ui/use-toast";
import { Input } from "./ui/input";
import { Download, Plus, Trash2, ExternalLink } from "lucide-react";
import { PipeConfigForm } from "./pipe-config-form";
import { useHealthCheck } from "@/lib/hooks/use-health-check";
import posthog from "posthog-js";
const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { readFile } from "@tauri-apps/plugin-fs";
import { homeDir, join } from "@tauri-apps/api/path";
import { convertHtmlToMarkdown } from "@/lib/utils";

export interface Pipe {
  enabled: boolean;
  id: string;
  source: string;
  fullDescription: string;
  config?: Record<string, any>;
}

interface CorePipe {
  id: string;
  description: string;
  url: string;
}

const corePipes: CorePipe[] = [
  {
    id: "pipe-phi3.5-engineering-team-logs",
    description: "continuously write logs of your days in a notion table using ollama+phi3.5",
    url: "https://github.com/mediar-ai/screenpipe/tree/main/examples/typescript/pipe-phi3.5-engineering-team-logs",
  },
  // {
  //   id: "transcriber",
  //   description: "generate accurate transcriptions",
  //   url: "https://github.com/screenpipe/transcriber-pipe",
  // },
];

const PipeDialog: React.FC = () => {
  const [newRepoUrl, setNewRepoUrl] = useState("");
  const [selectedPipe, setSelectedPipe] = useState<Pipe | null>(null);
  const [pipes, setPipes] = useState<Pipe[]>([]);
  const { settings, updateSettings } = useSettings();
  const { health } = useHealthCheck();
  useEffect(() => {
    fetchInstalledPipes();
  }, [health?.status]);

  const handleResetAllPipes = async () => {
    try {
      // stop screenpipe
      if (!settings?.devMode) {
        await invoke("kill_all_sreenpipes");
      }
      // reset pipes
      await invoke("reset_all_pipes");
      await new Promise((resolve) => setTimeout(resolve, 1000));
      toast({
        title: "All pipes deleted",
        description: "The pipes folder has been reset.",
      });
      // Refresh the pipe list and installed pipes
      await fetchInstalledPipes();
      setSelectedPipe(null);
    } catch (error) {
      console.error("Failed to reset pipes:", error);
      toast({
        title: "Error resetting pipes",
        description: "Please try again or check the logs for more information.",
        variant: "destructive",
      });
    } finally {
      setSelectedPipe(null);
      setPipes([]);
      // start screenpipe
      if (!settings?.devMode) {
        await invoke("spawn_screenpipe");
      }
    }
  };
  console.log("pipes", pipes);
  const fetchInstalledPipes = async () => {
    if (!health || health?.status === "error") {
      return;
    }

    try {
      const response = await fetch("http://localhost:3030/pipes/list");

      if (!response.ok) {
        throw new Error("failed to fetch installed pipes");
      }
      const data = await response.json();
      for (const pipe of data) {
        // read the README.md file from disk and set the fullDescription
        const home = await homeDir();
        const pathToReadme = await join(
          home,
          ".screenpipe",
          "pipes",
          pipe.id,
          "README.md"
        );
        const readme = await readFile(pathToReadme);
        const readmeString = new TextDecoder().decode(readme);
        pipe.fullDescription = convertHtmlToMarkdown(readmeString);
      }
      setPipes(data);
    } catch (error) {
      console.error("Error fetching installed pipes:", error);
      toast({
        title: "error fetching installed pipes",
        description: "please try again or check the logs for more information.",
        variant: "destructive",
      });
    }
  };
  const handleDownloadPipe = async (url: string) => {
    try {
      posthog.capture("download_pipe", {
        pipe_id: url,
      });
      toast({
        title: "downloading pipe",
        description: "please wait...",
      });
      const response = await fetch(`http://localhost:3030/pipes/download`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ url }),
      });
      if (!response.ok) {
        throw new Error("failed to download pipe");
      }
      const data = await response.json();
      toast({
        title: "pipe downloaded",
        // description: data.message,
      });
      // Refresh the pipe list
      // await addCustomPipe(url);
      await fetchInstalledPipes();
    } catch (error) {
      console.error("Failed to download pipe:", error);
      toast({
        title: "error downloading pipe",
        description: "please try again or check the logs for more information.",
        variant: "destructive",
      });
    }
  };

  const handleToggleEnabled = async (pipe: Pipe) => {
    try {
      posthog.capture("toggle_pipe", {
        pipe_id: pipe.id,
        enabled: !pipe.enabled,
      });
      if (!pipe.enabled) {
        // Enable the pipe through API
        await fetch(`http://localhost:3030/pipes/enable`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ pipe_id: pipe.id }),
        });

        toast({
          title: "enabling pipe",
          description: "this may take a few moments...",
        });
      } else {
        // Disable the pipe through API
        await fetch(`http://localhost:3030/pipes/disable`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ pipe_id: pipe.id }),
        });

        toast({
          title: "disabling pipe",
          description: "this may take a few moments...",
        });
      }

      await new Promise((resolve) => setTimeout(resolve, 1000));

      toast({
        title: pipe.enabled ? "pipe disabled" : "pipe enabled",
        description:
          "screenpipe has been updated with the new configuration. please restart screenpipe now in status badge",
      });

      // Update selectedPipe if it's the one being toggled
      if (selectedPipe && selectedPipe.id === pipe.id) {
        setSelectedPipe((prevPipe) =>
          prevPipe ? { ...prevPipe, enabled: !prevPipe.enabled } : null
        );
      }
    } catch (error) {
      console.error("Failed to toggle pipe:", error);
      toast({
        title: "error toggling pipe",
        description: "please try again or check the logs for more information.",
        variant: "destructive",
      });
    } finally {
      await fetchInstalledPipes();
    }
  };

  const handleAddOwnPipe = async () => {
    posthog.capture("add_own_pipe", {
      newRepoUrl,
    });
    if (newRepoUrl) {
      try {
        toast({
          title: "Adding custom pipe",
          description: "Please wait...",
        });
        // use /download endpoint to download the pipe
        const response = await fetch(`http://localhost:3030/pipes/download`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ url: newRepoUrl }),
        });
        if (!response.ok) {
          throw new Error("failed to download pipe");
        }
        const data = await response.json();
        // refresh the pipe list
        await fetchInstalledPipes();
        toast({
          title: "Custom pipe added",
          description:
            "Your pipe has been successfully added. Screenpipe will restart with the new pipe.",
        });
      } catch (error) {
        console.error("Failed to add custom pipe:", error);
        toast({
          title: "Error adding custom pipe",
          description: "Please check the URL and try again.",
          variant: "destructive",
        });
      } finally {
        setNewRepoUrl("");
        setSelectedPipe(null);
      }
    }
  };

  const handleConfigSave = async (config: Record<string, any>) => {
    if (selectedPipe) {
      fetch(`http://localhost:3030/pipes/update`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ pipe_id: selectedPipe.id, config }),
      });
      toast({
        title: "Configuration saved",
        description: "The pipe configuration has been updated.",
      });
    }
  };

  const formatUpdatedTime = (date: string) => {
    const now = new Date();
    const updated = new Date(date);
    const diffTime = Math.abs(now.getTime() - updated.getTime());
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    return `${diffDays} day${diffDays > 1 ? "s" : ""} ago`;
  };

  const renderPipeContent = () => {
    if (!selectedPipe) {
      return (
        <div className="flex flex-col items-center justify-center h-full">
          <p className="text-lg mb-4">no pipe selected</p>
          {/* <FeatureRequestLink /> */}
          {!health ||
            (health?.status === "error" && (
              <p className="mt-4 text-sm text-gray-500 text-center">
                screenpipe is not running.
                <br />
                please start screenpipe to use the pipe store.
              </p>
            ))}
        </div>
      );
    }

    return (
      <>
        <h2 className="text-2xl font-bold mb-2">{selectedPipe.id}</h2>
        <div className="flex justify-between items-center mb-4">
          <div>
            <a
              href={selectedPipe.source}
              target="_blank"
              rel="noopener noreferrer"
              className="underline"
            >
              repository
            </a>
          </div>
        </div>

        <div className="flex space-x-2 mb-4">
          <Button
            onClick={() => handleToggleEnabled(selectedPipe)}
            variant={selectedPipe.enabled ? "default" : "outline"}
            disabled={health?.status === "error"}
          >
            {selectedPipe.enabled ? "disable" : "enable"}
          </Button>

          <Button disabled variant="outline">
            copy share link
            <Badge variant="secondary" className="ml-2">
              soon
            </Badge>
          </Button>
          <Button disabled variant="outline">
            donate
            <Badge variant="secondary" className="ml-2">
              soon
            </Badge>
          </Button>
        </div>
        <Separator className="my-4" />

        {selectedPipe.enabled && (
          <>
            <PipeConfigForm
              pipe={selectedPipe}
              onConfigSave={handleConfigSave}
            />
            <Separator className="my-4" />
          </>
        )}

        {selectedPipe.fullDescription && (
          <div className="mt-4">
            <h3 className="text-xl font-semibold mb-2">about this pipe</h3>
            <MemoizedReactMarkdown
              className="prose break-words dark:prose-invert prose-p:leading-relaxed prose-pre:p-0 w-full"
              remarkPlugins={[remarkGfm, remarkMath]}
              components={{
                p({ children }) {
                  return <p className="mb-2 last:mb-0">{children}</p>;
                },
                code({ node, className, children, ...props }) {
                  const content = String(children).replace(/\n$/, "");
                  const match = /language-(\w+)/.exec(className || "");

                  if (!match) {
                    return (
                      <code
                        className="py-0.5 rounded-sm font-mono text-sm"
                        {...props}
                      >
                        {content}
                      </code>
                    );
                  }

                  return (
                    <CodeBlock
                      key={Math.random()}
                      language={(match && match[1]) || ""}
                      value={content}
                      {...props}
                    />
                  );
                },
                img({ src, alt }) {
                  return (
                    <img
                      src={src}
                      alt={alt}
                      className="max-w-full h-auto"
                      onError={(e) => {
                        const target = e.target as HTMLImageElement;
                        target.onerror = null;
                        target.src = "path/to/fallback/image.png";
                      }}
                    />
                  );
                },
              }}
            >
              {selectedPipe.fullDescription.replace(/Â/g, "")}
            </MemoizedReactMarkdown>
          </div>
        )}
      </>
    );
  };

  const renderCorePipes = () => (
    <div className="mb-3">
      <h3 className="text-lg font-semibold mb-2">try these pipes</h3>
      <div className="flex flex-col overflow-hidden">
        {corePipes.map((pipe) => (
          <Card key={pipe.id} className="p-4">
            <h4 className="font-medium text-lg mb-2">{pipe.id}</h4>
            <p className="text-sm text-gray-500 mb-4">{pipe.description}</p>
            <Button
              size="sm"
              className="w-full"
              onClick={() => handleDownloadPipe(pipe.url)}
            >
              <Download className="mr-2 h-4 w-4" />
              add pipe
            </Button>
          </Card>
        ))}
      </div>
    </div>
  );

  const renderContent = () => {
    if (!health || health?.status === "error") {
      return (
        <div className="flex flex-col items-center justify-center h-[500px]">
          <p className="text-lg mb-4 text-center">screenpipe is not running</p>
          <p className="text-sm text-gray-500 text-center">
            please start screenpipe to use the pipe store.
            <br />
            you can do this by clicking the status badge in the top right
            corner.
          </p>
        </div>
      );
    }
    return (
      <div className="flex flex-col h-[550px]">
        <div className="flex flex-1 overflow-hidden">
          <div className="w-3/5 pr-4 overflow-y-auto">
            {renderCorePipes()}
            <Separator className="my-4" />
            <h3 className="text-lg font-semibold mb-2">your pipes</h3>
            {pipes.map((pipe: Pipe) => (
              <Card
                key={pipe.id}
                className="cursor-pointer hover:bg-gray-100 mb-2 p-2"
                onClick={() => setSelectedPipe(pipe)}
              >
                <div className="flex justify-between items-start">
                  <h3>{pipe.id}</h3>
                </div>
              </Card>
            ))}
            <Card className="mb-2 p-2">
              <Input
                type="url"
                placeholder="enter repo url"
                value={newRepoUrl}
                onChange={(e) => setNewRepoUrl(e.target.value)}
              />
              <Button
                className="mt-2 w-full"
                onClick={handleAddOwnPipe}
                disabled={!newRepoUrl}
              >
                <Plus className="mr-2" size={16} />
                add your own pipe
              </Button>
            </Card>
          </div>
          <div className="w-full pl-4 border-l overflow-y-auto">
            {renderPipeContent()}
          </div>
        </div>
      </div>
    );
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost">pipe store</Button>
      </DialogTrigger>
      <DialogContent className="max-w-[90vw] w-full max-h-[90vh] h-full ">
        <DialogHeader>
          <DialogTitle>
            pipe store
            <Badge variant="secondary" className="ml-2">
              experimental
            </Badge>
          </DialogTitle>
          <div className="absolute top-4 right-20">
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    disabled={health?.status === "error"}
                    size="sm"
                    onClick={handleResetAllPipes}
                  >
                    <Trash2 className="mr-2 h-4 w-4" />
                    reset all pipes
                  </Button>
                </TooltipTrigger>
                <TooltipContent>
                  <p>use this if running into issues with the pipe store</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>

          <DialogDescription>
            screenpipe&apos;s store is a collection of plugins called
            &quot;pipes&quot; that are available to install.
            <br />
            it will process, annotate, help you search, automate in your
            screenpipe&apos;s data, or anything else you can imagine that help
            you get more out of your recordings.
            <br />
            make sure to restart screenpipe after changing a pipe&apos;s
            configuration.
            <a
              href="https://docs.screenpi.pe/docs/plugins"
              className="text-blue-500 hover:underline"
              target="_blank"
              rel="noopener noreferrer"
            >
              {" "}
              read the docs
            </a>
          </DialogDescription>

          {/* {selectedPipe && <FeatureRequestLink className="w-80" />} */}
        </DialogHeader>
        {/* center message in big */}
        {/* <div className="flex flex-col justify-center items-center h-[500px]">
          <p className="text-center">
            currently you need to enable pipes through `screenpipe pipe`
            commands or `/pipes` api
            <br />
            we&apos;re going to make this nontechnical next week.
          </p>
          <br />
          <a
            href="https://github.com/mediar-ai/screenpipe/tree/main/examples/typescript"
            className="text-blue-500 hover:underline"
            target="_blank"
            rel="noopener noreferrer"
          >
            check out more examples on github
          </a>
        </div> */}
        {renderContent()}
      </DialogContent>
    </Dialog>
  );
};

export default PipeDialog;
