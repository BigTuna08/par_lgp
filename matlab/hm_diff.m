function hm_diff(root_dir)
%UNTITLED2 Summary of this function goes here
%   Detailed explanation goes here

%max_v = max(M(:));
%M = M/max_v;
M = get_ave_map_diff(root_dir);
min_v = min(min(M))
max_v = max(max(M))


h = heatmap(M);
caxis([min_v, max_v])
h.Colormap = parula;
h.XData = 0:49;
h.YData = 0:49;

warning('off','MATLAB:structOnObject') % for setting x axis to top
axp = struct(h);     
axp.Axes.XAxisLocation = 'top';
set(gcf,'units','normalized','outerposition',[0 0 1 1]); %full screen

end
