import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/lib/utils";
import { ChevronLeft, ChevronRight, Plus, Trash2 } from "lucide-react";
import { formatDistanceToNow } from "date-fns";
import { SearchHistory } from "@/lib/types/history";
import { buttonVariants } from "@/components/ui/button";

interface SearchHistorySidebarProps {
  searches: SearchHistory[];
  currentSearchId: string | null;
  onSelectSearch: (id: string) => void;
  onDeleteSearch: (id: string) => void;
  isCollapsed: boolean;
  onToggleCollapse: () => void;
  onNewSearch?: () => void;
}

export default function SearchHistorySidebar({
  searches,
  currentSearchId,
  onSelectSearch,
  onDeleteSearch,
  isCollapsed,
  onToggleCollapse,
  onNewSearch,
}: SearchHistorySidebarProps) {
  return (
    <div 
      className={cn(
        "border-r h-screen flex flex-col transition-all duration-200 bg-background z-50",
        isCollapsed ? "w-[50px]" : "w-[300px]"
      )}
    >
      <div className="p-4 border-b flex justify-between items-center">
        {!isCollapsed && <h2 className="text-sm font-semibold">search history</h2>}
        <Button 
          variant="ghost" 
          size="icon"
          onClick={onToggleCollapse}
          className="ml-auto"
        >
          {isCollapsed ? <ChevronRight className="h-4 w-4" /> : <ChevronLeft className="h-4 w-4" />}
        </Button>
      </div>
      {!isCollapsed && (
        <>
          <div className="px-2 py-2">
            <Button
              onClick={onNewSearch}
              className={cn(
                buttonVariants({ variant: 'outline' }),
                'h-10 w-full justify-start bg-zinc-50 px-4 shadow-none transition-colors hover:bg-zinc-200/40 dark:bg-zinc-900 dark:hover:bg-zinc-300/10'
              )}
            >
              <Plus className="mr-2 h-4 w-4" />
              new search
            </Button>
          </div>
          <ScrollArea className="flex-1">
            <div className="p-2 space-y-2">
              {searches.map((search) => (
                <div
                  key={search.id}
                  className={cn(
                    "p-3 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer group relative",
                    currentSearchId === search.id &&
                      "bg-gray-100 dark:bg-gray-800"
                  )}
                  onClick={() => onSelectSearch(search.id)}
                >
                  <h3 className="text-sm font-medium truncate">
                    {search.query || "untitled search"}
                  </h3>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    {formatDistanceToNow(new Date(search.timestamp), {
                      addSuffix: true,
                    })}
                  </p>
                  {search.messages.length > 1 && (
                    <p className="text-xs text-gray-400 mt-1">
                      {search.messages.length} messages
                    </p>
                  )}
                  <Button
                    variant="ghost"
                    size="icon"
                    className="absolute right-2 top-2 opacity-0 group-hover:opacity-100 transition-opacity"
                    onClick={(e) => {
                      e.stopPropagation();
                      onDeleteSearch(search.id);
                    }}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          </ScrollArea>
        </>
      )}
    </div>
  );
} 