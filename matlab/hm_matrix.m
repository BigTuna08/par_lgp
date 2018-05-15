function hm_file(M)
%UNTITLED2 Summary of this function goes here
%   Detailed explanation goes here

%max_v = max(M(:));
%M = M/max_v;
min_v = min(min(M(M>0)))
max_v = max(max(M))


h = heatmap(M);
caxis([max_v-0.05, max_v])
h.Colormap = parula;
h.XData = 1:25;
h.YData = 1:25;

warning('off','MATLAB:structOnObject') % for setting x axis to top
axp = struct(h);     
axp.Axes.XAxisLocation = 'top';
set(gcf,'units','normalized','outerposition',[0 0 1 1]); %full screen

end

